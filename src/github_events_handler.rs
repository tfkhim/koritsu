use axum::Json;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct WorkflowRun {
    name: String,
    conclusion: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct GithubWorkflowRunEvent {
    action: String,
    workflow_run: WorkflowRun,
}

pub async fn handle_github_workflow_run(Json(payload): Json<GithubWorkflowRunEvent>) {
    println!(
        "Received workflow run event for workflow '{}'",
        payload.workflow_run.name
    );
    println!("Action is '{}'", payload.action);
    match payload.workflow_run.conclusion {
        Some(conclusion) => println!("Conclusion: {}", conclusion),
        None => println!("Conclusion: Not available yet"),
    }
}
