

use std::io;

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

	

	if tasks.iter().any(|task| task.title == old_title) {

		if tasks.iter().any(|task| task.title == new_title)
			&& old_title != new_title {
			return AlreadyExistsForRename;
		}

	} else {
		return NotFoundForRename;
	}

	let t = tasks.iter_mut().find(|t| t.title == old_title);

	if let Some(task) = t {
		task.title = new_title.to_string();
	}

	Renamed
}

fn display_complete_task_result(result: &CompleteTaskResult) {
	match result {
		CompleteTaskResult::Completed => println!("Tâche terminée avec succès !"),
		CompleteTaskResult::AlreadyCompleted => println!("Cette tâche était déjà terminée."),
		CompleteTaskResult::NotFound => println!("Tâche non trouvée"),
	}
}

fn display_add_task_result(result: &AddTaskResult) {
	match result {
		Added => println!("Tâche ajoutée avec succès !"),
		AlreadyExists => println!("Tâche existante")
	}
}

fn display_remove_task_result(result: &RemoveTaskResult) {
	match result {
		Removed => println!("Tâche supprimée avec succès !"),
		NotFoundForRemove => println!("Tâche non trouvée")
	}
}

fn display_rename_task_result(result: &RenameTaskResult) {
	match result {
		Renamed => println!("Tâche renommée avec succès !"),
		NotFoundForRename => println!("Tâche non trouvée."),
		AlreadyExistsForRename => println!("Tâche existante avec ce titre.")
	}
}

fn read_user_input() -> Result<String, io::Error> {
	let mut input = String::new();

	// println!("Entrez le titre de la tâche :");

	match io::stdin().read_line(&mut input) {
		Ok(_n) => {
			// println!("{n} bytes read");
			// println!("{input}");
			Ok(input.trim().to_string())
		}
		Err(error) => {
			// eprint!("erreur: {error}");
			Err(error)
		},
	}
}

fn main() {

	// 1. Créer un Vec<Task> vide.
	let mut tasks: Vec<Task> = Vec::new();

	loop {
		// Affichage du menu
		println!("=== TaskForge ===");
		println!("1. Ajouter une tâche");
		println!("2. Afficher les tâches");
		println!("3. Terminer une tâche");
		println!("4. Supprimer une tâche");
		println!("5. Renommer une tâche");
		println!("6. Quitter");
		println!("Votre choix : ");

		// 2. Demander à l'utilisateur le titre de la tâche.
		// En appelant read_user_input() et gérer les 2 cas avec match
		let result = read_user_input();
		match result {
			Ok(value) => {
				match value.as_str() {
					"1" => {
						println!("Entrez le titre de la tâche :");
						let input = read_user_input();
						match input {
							Ok(title) => {
								let result = add_task(&mut tasks, &title);
								display_add_task_result(&result);
							}
							Err(error) => {
								eprintln!("Erreur lors de l'ajout de la tâche : {}", error);
							}
						}
					}
					"2" => {
						println!("Liste des tâches :");
						list_tasks(&tasks);
					}
					"3" => {
						println!("Entrez le titre de la tâche à terminer :");
						let input = read_user_input();
						match input {
							Ok(title) => {
								let result = complete_task(&mut tasks, &title);
								display_complete_task_result(&result);	
							}
							Err(error) => {
								eprintln!("Erreur lors de la terminaison de la tâche : {error}");
							}
						}
					}
					"4" => {
						println!("Entrez le titre de la tâche à supprimer :");
						let input = read_user_input();
						match input {
							Ok(title) => {
								let result = remove_task(&mut tasks, &title);
								display_remove_task_result(&result);
							}
							Err(error) => {
								eprintln!("Erreur lors de la suppression de la tâche : {error}");
							}
						}
					}
					"5" => {
						println!("Entrez le titre de la tâche à renommer :");
						let first_input = read_user_input();

						match first_input {
							Ok(old_title) => {
								println!("Entrez le nouveau titre de la tâche :");
								let second_input = read_user_input();
								match second_input {
									Ok(new_title) => {
										let result = rename_task(&mut tasks, &old_title, &new_title);
										display_rename_task_result(&result);
									}
									Err(error) => {
										eprintln!("Erreur lors du renommage de la tâche : {error}")
									}
								}
							}
							Err(error) => {
								eprintln!("Erreur lors du renommage de la tâche : {error}");
							}
						}
					}
					"6" => {
						println!("Vous quittez le programme...");
						break;
					}
					_ => {
						println!("Choix invalide.");
					},
				}
			}
			Err(error) => {
				eprintln!("Erreur lors de la lecture de l'entrée de l'utilisateur : {error}");
				return;
			}
		}
	}
}
