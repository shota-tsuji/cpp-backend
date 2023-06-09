use std::collections::{HashMap, HashSet};
use async_graphql::{Context, EmptyMutation, EmptySubscription, ID, Object, Schema};
use sqlx::mysql::MySqlPool;

use crate::presentation::graphql::mutation::Mutation;
use crate::presentation::graphql::object::{HelloResponse, Process, Resource, ResourceInfo, StepResult};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

use hello_world::greeter_client::GreeterClient;
use hello_world::{HelloRequest, ProcessRequest};
use crate::presentation::graphql::query::hello_world::StepOutput;

use super::object::{Recipe, RecipeDetail, Step};

pub type QuerySchema = Schema<Query, Mutation, EmptySubscription>;

pub struct Query {
    pool: MySqlPool,
}

impl Query {
    pub fn new(pool: MySqlPool) -> Self {
        Query { pool }
    }
}

#[Object]
impl Query {
    async fn recipe_detail(&self, _ctx: &Context<'_>, id: ID) -> Result<RecipeDetail, String> {
        let recipe_row: Option<RecipeRow> =
            sqlx::query_as(r#"SELECT id, title, description FROM recipes WHERE id = ?"#)
                .bind(id.as_str())
                .fetch_optional(&self.pool)
                .await
                .unwrap();

        let recipe = recipe_row
            .map(|row| Recipe {
                id: row.id,
                title: row.title,
                description: row.description,
            })
            .unwrap();

        let steps = sqlx::query_as("select id, description, resource_id, order_number, duration from steps where recipe_id = ?")
            .bind(id.as_str())
            .fetch_all(&self.pool)
            .await.unwrap().into_iter().map(|row: StepRow| {
            let id = row.id;
            let description = row.description;
            let resource_id = row.resource_id;
            let order_number = row.order_number;
            let duration = row.duration;
            Step {
                id,
                description,
                resource_id,
                order_number,
                duration,
            }
        }).collect();

        let recipe_detail = RecipeDetail {
            id: recipe.id,
            title: recipe.title,
            description: recipe.description,
            steps,
        };

        println!("{:?}", &recipe_detail);

        Ok(recipe_detail)
    }

    async fn recipes(&self, _ctx: &Context<'_>) -> Result<Vec<Recipe>, String> {
        let recipes = sqlx::query_as("SELECT * FROM recipes")
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row: RecipeRow| {
                let id = row.id;
                let title = row.title;
                let description = row.description;
                println!("{:?}, {:?}, {:?}", id, title, description);
                Recipe {
                    id,
                    title,
                    description,
                }
            })
            .collect();

        Ok(recipes)
    }

    async fn process(&self, _ctx: &Context<'_>, id: ID) -> Result<Process, String> {
        let mut client = GreeterClient::connect("http://main:50051").await.unwrap();

        println!("step0");
        let recipes: Vec<Recipe> = sqlx::query_as(r#"SELECT r.id, r.title, r.description FROM process_regsitrations AS pr LEFT JOIN recipes AS r ON pr.recipe_id = r.id WHERE pr.process_id = ?"#)
            .bind(id.as_str())
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row: RecipeRow| {
                let id = row.id;
                let title = row.title;
                let description = row.description;
                println!("{:?}, {:?}, {:?}", id, title, description);
                Recipe {
                    id,
                    title,
                    description,
                }
            })
            .collect();

        println!("step1");
        let mut resource_set = HashSet::new();
        let mut grpc_recipes: Vec<hello_world::Recipe> = Vec::new();
        let mut step_infos: HashMap<String, String> = HashMap::new();
        let mut recipe_infos: HashMap<String, String> = HashMap::new();
        for recipe in &recipes {
            let steps: Vec<Step> = sqlx::query_as("select id, description, resource_id, order_number, duration from steps where recipe_id = ?")
                .bind(recipe.id.as_str())
                .fetch_all(&self.pool)
                .await.unwrap().into_iter().map(|row: StepRow| {
                let id = row.id;
                let description = row.description;
                let resource_id = row.resource_id;
                let order_number = row.order_number;
                let duration = row.duration;
                Step {
                    id,
                    description,
                    resource_id,
                    order_number,
                    duration,
                }
            }).collect();

            println!("step2");
            recipe_infos.insert(recipe.id.clone(), recipe.title.clone());

            // Get unique resource ids
            for step in &steps {
                resource_set.insert(step.resource_id);
                step_infos.insert(step.id.clone(), step.description.clone());
            }

            let grpc_steps = steps.iter().map(|step| {
                hello_world::Step {
                    id: step.id.clone(),
                    recipe_id: recipe.id.clone(),
                    resource_id: step.resource_id.clone(),
                    duration: step.duration,
                    order_number: step.order_number,
                }
            }).collect();
            grpc_recipes.push(hello_world::Recipe {
                id: recipe.id.clone(),
                steps: grpc_steps
            });
        }

        let mut resources: Vec<Resource> = sqlx::query_as("SELECT * FROM resources")
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row: ResourceRow| {
                let id = row.id;
                let name = row.name;
                let amount = row.amount;
                println!("{:?}, {:?}, {:?}", id, name, amount);
                Resource {
                    id,
                    name,
                    amount,
                }
            }).collect();
        resources.sort_by_key(|r| r.id);
        println!("{:?}", resources);

        println!("step3");
        // Filter only used resources
        let grpc_resources: Vec<hello_world::Resource> = resources.iter().filter(|&resource| resource_set.contains(&resource.id)).map(|resource|{
            hello_world::Resource {
                id: resource.id,
                amount: resource.amount
            }
        }).collect();

        let request = tonic::Request::new(ProcessRequest {
            recipes: grpc_recipes.into(),
            resources: grpc_resources.into(),
        });

        let mut response = client.process(request).await.unwrap();
        println!("{:?}", response.get_ref().steps);
        println!("{:?}", response.get_ref().resource_infos);

        let step_results: Vec<StepResult> = response.get_ref().steps.iter().map(|step: &StepOutput| {
            let description = step_infos.get(step.step_id.as_str()).unwrap();
            let recipe_name = recipe_infos.get(step.recipe_id.as_str()).unwrap();
            StepResult {
                id: step.step_id.clone(),
                recipe_id: step.recipe_id.clone(),
                resource_id: step.resource_id,
                start_time: step.start_time,
                duration: step.duration,
                order_number: 0,
                timeline_index: step.time_line_index,
                description: description.to_string(),
                recipe_name: recipe_name.to_string(),
            }
        }).collect();

        response.get_mut().resource_infos.sort_by_key(|r| r.id);
        let mut resource_infos: Vec<ResourceInfo> = Vec::new();
        for (i, resource) in response.get_ref().resource_infos.iter().enumerate() {
            for j in 0..resource.used_resources_count {
                let mut steps: Vec<StepResult> = Vec::new();
                for step in &step_results {
                    if step.resource_id == resource.id as u64 && j == step.timeline_index {
                        steps.push(step.clone());
                    }
                }

                resource_infos.push(ResourceInfo{
                    id: resource.id as u64,
                    name: resources[i].name.clone(),
                    steps,
                });
            }
        }
        println!("{:?}", resource_infos);

        let process = Process {
            resource_infos,
        };

        println!("step4");
        //Ok(recipeDetails)
        Ok(process)
    }

    async fn resource(&self, _ctx: &Context<'_>, id: ID) -> Result<Resource, String> {
        let resource_row: Option<ResourceRow> = sqlx::query_as(r#"SELECT id, name, amount FROM resources WHERE id = ?"#)
            .bind(id.as_str())
            .fetch_optional(&self.pool)
            .await
            .unwrap();

        let resource = resource_row.map(|row| Resource {
            id: row.id,
            name: row.name,
            amount: row.amount,
        }).unwrap();

        println!("{:?}", &resource);

        Ok(resource)
    }

    async fn resources(&self, _ctx: &Context<'_>) -> Result<Vec<Resource>, String> {
        let resources = sqlx::query_as("SELECT * FROM resources")
            .fetch_all(&self.pool)
            .await
            .unwrap()
            .into_iter()
            .map(|row: ResourceRow| {
                let id = row.id;
                let name = row.name;
                let amount = row.amount;
                println!("{:?}, {:?}, {:?}", id, name, amount);
                Resource {
                    id,
                    name,
                    amount,
                }
            }).collect();

        Ok(resources)
    }
}

#[derive(sqlx::FromRow)]
struct RecipeRow {
    id: String,
    title: String,
    description: String,
}

#[derive(sqlx::FromRow)]
struct StepRow {
    id: String,
    description: String,
    resource_id: u64,
    order_number: u32,
    duration: i32,
}

#[derive(sqlx::FromRow)]
struct ResourceRow {
    pub id: u64,
    pub name: String,
    pub amount: i32,
}
