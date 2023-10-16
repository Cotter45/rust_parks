use std::fs::File;
use std::io::BufReader;
use actix_web::web::Data; 
use utoipa::{ToSchema, OpenApi};
use utoipa_swagger_ui::SwaggerUi;
use serde::{Deserialize, Serialize};
use actix_web::{web, App, HttpServer, HttpResponse, Responder, get};


#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
struct State {
    id: u32,
    state: String,
    total_parks: Option<u32>,
    exclusive_parks: Option<u32>,
    shared_parks: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)] 
#[serde(rename_all = "camelCase")]
struct Park {
    id: u32,
    name: String,
    image: String,
    location: String,
    established: String,
    area: String,
    visitors: u32,
    description: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
struct Response {
    pub status: &'static str,
    pub message: String,
}

struct AppState {
    parks: Vec<Park>,
    states: Vec<State>,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            parks: load_parks(),
            states: load_states(),
        }
    }
}

#[derive(OpenApi)]
#[openapi(
    paths(
        get_all_parks,
        get_park_by_id,
        find_parks_by_query,
        get_all_states,
        get_state_by_id,
        find_states_by_query,
    ),
    components(
        schemas(Response, Park, State)
    ),
    tags(
        (name = "Parks", description = "National Parks"),
        (name = "States", description = "States with National Parks"),
    ),
)]
struct ApiDoc;


fn load_parks() -> Vec<Park> {
    let file = File::open("./data/parks.json").expect("Failed to open parks.json");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to parse parks.json")
}

fn load_states() -> Vec<State> {
    let file = File::open("./data/states.json").expect("Failed to open states.json");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).expect("Failed to parse states.json")
}

fn fuzzy_match(query: &str, target: &str) -> bool {
    let query = query.to_lowercase();
    let target = target.to_lowercase();
    
    if target.contains(&query) {
        return true;
    }

    // Split the strings into tokens (words)
    let target_tokens: Vec<&str> = target.split_whitespace().collect();
    let query_tokens: Vec<&str> = query.split_whitespace().collect();
    
    for q_token in query_tokens {
        for t_token in &target_tokens {
            if t_token.contains(q_token) {
                return true;
            }
        }
    }
    
    false
}

#[utoipa::path(
    get,
    path = "/parks",
    tag = "Parks",
    responses(
        (status = 200, description= "Get all parks", body = Vec<Park>),
        (status = 500, description= "Internal server error", body = Response),
    )
)]
#[get("/parks")]
async fn get_all_parks(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&data.parks)
}

#[utoipa::path(
    get,
    path = "/parks/{id}",
    tag = "Parks",
    responses(
        (status = 200, description= "Get park by ID", body = Park),  
        (status = 404, description= "Park not found", body = Response),  
        (status = 500, description= "Internal server error", body = Response), 
    )
)]
#[get("/parks/{id}")]
async fn get_park_by_id(info: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let id: u32 = info.into_inner();
    if let Some(park) = data.parks.iter().find(|p| p.id == id) {
        HttpResponse::Ok().json(park)
    } else {
        HttpResponse::NotFound().json(Response {
            status: "error",
            message: "Park not found".to_string(),
        })
    }
}

#[utoipa::path(
    get,
    path = "/parks/search/{query}",
    tag = "Parks",
    responses(
        (status = 200, description= "Find parks by query", body = Vec<Park>),   
        (status = 500, description= "Internal server error", body = Response),  
    )
)]
#[get("/parks/search/{query}")]
async fn find_parks_by_query(info: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let query = info.into_inner();
    let mut results = vec![];
    for park in &data.parks {
        if fuzzy_match(&query, &park.name) || fuzzy_match(&query, &park.description) {
            results.push(park.clone());
        }
    }
    HttpResponse::Ok().json(results)
}

#[utoipa::path(
    get,
    path = "/states",
    tag = "States",
    responses(
        (status = 200, description= "Get all states", body = State),
        (status = 500, description= "Internal server error", body = Response), 
    )
)]
#[get("/states")]
async fn get_all_states(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&data.states)
}

#[utoipa::path(
    get,
    path = "/states/{id}",
    tag = "States",
    responses(
        (status = 200, description= "Get state by ID", body = State),
        (status = 404, description= "State not found", body = Response),
        (status = 500, description= "Internal server error", body = Response),   
    )
)]
#[get("/states/{id}")]
async fn get_state_by_id(info: web::Path<u32>, data: web::Data<AppState>) -> impl Responder {
    let id: u32 = info.into_inner();
    if let Some(state) = data.states.iter().find(|s| s.id == id) {
        HttpResponse::Ok().json(state)
    } else {
        HttpResponse::NotFound().json(Response {
            status: "error",
            message: "State not found".to_string(),
        })
    }
}

#[utoipa::path(
    get,
    path = "/states/search/{query}",
    tag = "States",
    responses(
        (status = 200, description= "Find states by query", body = Response), 
        (status = 500, description= "Internal server error", body = Response),    
    )
)] 
#[get("/states/search/{query}")]
async fn find_states_by_query(info: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let query = info.into_inner();
    let mut results = vec![];
    for state in &data.states {
        if fuzzy_match(&query, &state.state) {
            results.push(state.clone());
        }
    }
    HttpResponse::Ok().json(results)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let openapi = ApiDoc::openapi();

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState::new()))
            .service(get_all_parks)
            .service(get_park_by_id)
            .service(find_parks_by_query)
            .service(get_all_states)
            .service(get_state_by_id)
            .service(find_states_by_query)
            .service(
                SwaggerUi::new("/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}