use crate::{AddTaskResult::{Added, AlreadyExists}, CompleteTaskResult::{AlreadyCompleted, Completed, NotFound}, RemoveTaskResult::{NotFound as NotFoundForRemove, Removed}, RenameTaskResult::{AlreadyExists as AlreadyExistsForRename, NotFound as NotFoundForRename, Renamed}};

struct Task {
	title: String,
	completed: bool
}

enum AddTaskResult {
	Added,
	AlreadyExists
}

enum RemoveTaskResult {
	Removed,
	NotFound
}

enum CompleteTaskResult {
	Completed,
	AlreadyCompleted,
	NotFound
}

enum RenameTaskResult {
	Renamed,
	NotFound,
	AlreadyExists
}

fn add_task(tasks: &mut Vec<Task>, title: &str) -> AddTaskResult {

	for task in tasks.iter() {
		if task.title == title {
			return AlreadyExists;
		}
	}

	tasks.push(Task {
		title: String::from(title),
		completed: false
	});

	Added
}

fn remove_task(tasks: &mut Vec<Task>, title: &str) -> RemoveTaskResult {

	for (index, task) in tasks.iter().enumerate() {
		if task.title == title {
			tasks.remove(index);
			return Removed;
		}
	}

	NotFoundForRemove
}

fn complete_task(tasks: &mut Vec<Task>, title: &str) -> CompleteTaskResult {
	for task in tasks {
		if task.title == title {
			if task.completed {
				return AlreadyCompleted;
			} else {
				task.completed = true;
				return Completed;
			}
		}
	}

	NotFound
}

fn list_tasks(tasks: &[Task]) {
	let mut i = 0;
	
	for task in tasks {
		i += 1;
	
		print!("{}. {} -- ", i, task.title);
		if task.completed {
			println!("Terminée");
		} else {
			println!("En cours")
		}
	}
}

fn rename_task(
	tasks: &mut Vec<Task>,
	old_title: &str,
	new_title: &str
) -> RenameTaskResult {

	for task in tasks.iter() {

		if old_title == new_title && 
			task.title == old_title {
				return  Renamed;
		}

		if task.title == new_title {
			return AlreadyExistsForRename;
		}
		
	}

	for task in tasks.iter_mut() {
		if task.title == old_title {
			task.title = new_title.to_string();
			return Renamed;
		}
	}

	NotFoundForRename
}

fn complete_task_result_match(result: &CompleteTaskResult) {
	match result {
		CompleteTaskResult::Completed => println!("Tâche terminée avec succès !"),
		CompleteTaskResult::AlreadyCompleted => println!("Cette tâche était déjà terminée."),
		CompleteTaskResult::NotFound => println!("Tâche non trouvée"),
	}
}

fn add_task_result_match(result: &AddTaskResult) {
	match result {
		Added => println!("Tâche ajoutée avec succès !"),
		AlreadyExists => println!("Tâche existante")
	}
}

fn remove_task_match(result: &RemoveTaskResult) {
	match result {
		Removed => println!("Tâche supprimée avec succès !"),
		NotFoundForRemove => println!("Tâche non trouvée")
	}
}

fn rename_task_result(result: RenameTaskResult) {
	match result {
		Renamed => println!("Tâche renommée avec succès !"),
		NotFoundForRename => println!("Tâche non trouvée."),
		AlreadyExistsForRename => println!("Tâche existante avec ce titre.")
	}
}

fn main() {
	println!("=== TaskForge ===");

	let mut tasks: Vec<Task> = Vec::new();

	let add_result1 = add_task(&mut tasks, "Apprendre Rust");
	let add_result2 = add_task(&mut tasks, "Comprendre ownership");
	let add_result3 = add_task(&mut tasks, "Créer un projet");
	let add_result4 = add_task(&mut tasks, "Apprendre les enums");
	let add_result5 = add_task(&mut tasks, "Apprendre Rust");

	add_task_result_match(&add_result1);
	add_task_result_match(&add_result2);
	add_task_result_match(&add_result3);
	add_task_result_match(&add_result4);
	add_task_result_match(&add_result5);

	let complete_result1 = complete_task(&mut tasks, "Apprendre Rust");
	let complete_result2 = complete_task(&mut tasks, "Apprendre Rust");
	let complete_result3 = complete_task(&mut tasks, "Tâche inexistante");
	complete_task_result_match(&complete_result1);
	complete_task_result_match(&complete_result2);
	complete_task_result_match(&complete_result3);

	let remove_result1 = remove_task(&mut tasks, "Créer un projet");
	let remove_result2 = remove_task(&mut tasks, "Tâche inexistante");
	remove_task_match(&remove_result1);
	remove_task_match(&remove_result2);
	
	list_tasks(&tasks);

	println!("Nombre de tâches : {}", tasks.len());
}
