

use std::io;

use crate::{AddTaskResult::{Added, AlreadyExists}, CompleteTaskResult::{AlreadyCompleted, Completed, NotFound}, RemoveTaskResult::{NotFound as NotFoundForRemove, Removed}, RenameTaskResult::{AlreadyExists as AlreadyExistsForRename, InvalidTitle, NotFound as NotFoundForRename, Renamed}};

struct Task {
	title: String,
	completed: bool
}

#[derive(PartialEq, Debug)]
enum AddTaskResult {
	Added,
	AlreadyExists,
	InvalidTitle
}

#[derive(Debug, PartialEq)]
enum RemoveTaskResult {
	Removed,
	NotFound
}

#[derive(Debug, PartialEq)]
enum CompleteTaskResult {
	Completed,
	AlreadyCompleted,
	NotFound
}

#[derive(Debug, PartialEq)]
enum RenameTaskResult {
	Renamed,
	NotFound,
	AlreadyExists,
	InvalidTitle
}

fn add_task(tasks: &mut Vec<Task>, title: &str) -> AddTaskResult {

	if title.trim().is_empty() {
		return AddTaskResult::InvalidTitle;
	}

	let title = title.trim();

	for task in tasks.iter() {
		if task.title == title {
			return AlreadyExists;
		}
	}

	tasks.push(Task {
		title: String::from(title.trim()),
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
	if tasks.len() == 0 {
		println!("    Vide.");
	}

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
	if new_title.trim().is_empty() {
		return InvalidTitle;
	}	

	let new_title = new_title.trim();

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
		task.title = new_title.trim().to_string();
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
		AlreadyExists => println!("Tâche existante"),
		AddTaskResult::InvalidTitle => println!("Titre invalide"),
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
		AlreadyExistsForRename => println!("Tâche existante avec ce titre."),
		InvalidTitle => println!("Titre invalide")
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
						println!();
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


// Préparer les modules de tests
#[cfg(test)]
mod tests {

use super::*;

	#[test]
	fn add_task_adds_new_task() {

		// 1. Créer une liste de tâches vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Appeler add_task avec le titre "Apprendre Rust"
		let result = add_task(&mut tasks, "Apprendre Rust");

		// 3. Vérifier que le résultat est AddTaskResult::Added
		assert_eq!(result, AddTaskResult::Added);

		// 4. Vérifier que la liste contient maintenant une tâche
		assert_eq!(tasks.len(), 1);
	}

	#[test]
	fn add_task_rejects_duplicate_title() {

		// 1. Créer une liste de tâches vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter une première tâche intitulée "Apprendre Rust"
		let result = add_task(&mut tasks, "Apprendre Rust");

		// 3. Vérifier la première tentative d'ajout réussit
		assert_eq!(result, AddTaskResult::Added);

		// 4. Tenter d'ajouter une deuxième fois le même titre
		let result = add_task(&mut tasks, "Apprendre Rust");

		// 5. Vérifier que le résultat est AlreadyExists
		assert_eq!(result, AddTaskResult::AlreadyExists);

		// 6. V'rifier que la liste contient toujours une seule tâche
		assert_eq!(tasks.len(), 1);
	}

	#[test]
	fn complete_task_marks_task_as_completed() {
		// 1. Créer une liste de tâche vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter une tâche intitulé "Apprendre Rust"
		let result = add_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, AddTaskResult::Added);

		// 3. Appeler complete_task() avec ce titre 
		let result = complete_task(&mut tasks, "Apprendre Rust");

		// 4. Vérifier que le résultat est CompleteTaskResult::Completed
		assert_eq!(result, CompleteTaskResult::Completed);

		// 5. Vérifier que la tâche est maintenant terminée
		assert!(tasks[0].completed);
	}

	#[test]
	fn complete_task_rejects_already_completed_task() {

		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter la tâche "Apprendre Rust"
		let result = add_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, AddTaskResult::Added);

		// 3. Terminer la tâche une première fois
		let result = complete_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, CompleteTaskResult::Completed);

		// 4. Appeler complete_task() une deuxième fois
		let result = complete_task(&mut tasks, "Apprendre Rust");
		
		// 5. Vérifier que le résultat est AlreadyCompleted
		assert_eq!(result, CompleteTaskResult::AlreadyCompleted);

		// 6. Vérifier que la tâche reste terminée
		assert!(tasks[0].completed);
	}

	#[test]
	fn remove_task_removes_existing_task() {
		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter une tâche intitulée "Apprendre Rust"
		let result = add_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, AddTaskResult::Added);
		assert_eq!(tasks.len(), 1);

		// 3. Supprimer cette tâche 
		let result = remove_task(&mut tasks, "Apprendre Rust");

		// 4. Vérifier que le résultat est RemoveTaskResult::Removed
		assert_eq!(result, RemoveTaskResult::Removed);

		// 5. Vérifier que la liste est vide
		assert_eq!(tasks.len(), 0);
	}

	#[test]
	fn remove_task_returns_not_found_for_missing_task() {
		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter "Apprendre Rust"
		let result = add_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, AddTaskResult::Added);

		// 3. Essayer de supprimer "Apprendre Java"
		let result = remove_task(&mut tasks, "Apprendre Java");

		// 4. Vérifier que le résultat est RemoveTaskResult::NotFound
		assert_eq!(result, RemoveTaskResult::NotFound);

		// 5. Vérifier que la liste contient toujours une tâche
		assert_eq!(tasks.len(), 1);
	}

	#[test]
	fn rename_task_changes_title() {
		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter "Apprendre Rust"
		let result = add_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, AddTaskResult::Added);

		// 3. Renommer "Apprendre Rust" en "Maîtriser Rust"
		let result = rename_task(
			&mut tasks,
			"Apprendre Rust",
			"Maîtriser Rust");
		
		// 4. Vérifier que le résultat est RenameTaskResult::Renamed
		assert_eq!(result, RenameTaskResult::Renamed);

		// 5. Vérifier que l'ancien titre n'existe plus
		assert!(!tasks.iter().any(|task| task.title == "Apprendre Rust"));

		// 6. Vérifier que le nouveau titre existe
		assert!(tasks.iter().any(|task| task.title == "Maîtriser Rust"));
	}

	#[test]
	fn rename_task_rejects_existing_title() {
		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter "Apprendre Rust"
		add_task(&mut tasks, "Apprendre Rust");

		// 3. Ajouter "Apprendre Java"
		let result = add_task(&mut tasks, "Apprendre Java");
		assert_eq!(result, AddTaskResult::Added);

		// 4. Tenter de renommer "Apprendre Rust"
		// en "Apprendre Java"
		let result = rename_task(
			&mut tasks, 
			"Apprendre Rust", 
			"Apprendre Java");
			
		// 5. Vérifier que le résultat est AlreadyExists
		assert_eq!(result, RenameTaskResult::AlreadyExists);
		
		// 6. Vérifier que les deux tâches sont toujours présentes
		assert_eq!(tasks.len(), 2);

		// 7. Vérifier que leurs titres n'ont pas changé
		assert!(tasks.iter().any(|task| task.title == "Apprendre Rust"));
		assert!(tasks.iter().any(|task| task.title == "Apprendre Java"));

	}

	#[test]
	fn rename_task_returns_not_found_for_missing_task() {
		// 1. Crée un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter une tâche intitulée "Apprendre Rust"
		let result = add_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, AddTaskResult::Added);

		// 3. Teneter de renommer "Apprendre Java" en "Maîtriser Java"
		let result = rename_task(
			&mut tasks, 
			"Apprendre Java", 
			"Maîtriser Java");
		
		// 4. Vérifier que le résultat est RenameTaskResult::NotFound
		assert_eq!(result, RenameTaskResult::NotFound);

		// 5. Vérifier que le vecteur contient toujours une seule tâche
		assert_eq!(tasks.len(), 1);

		// 6. Vérifier que "Apprendre Rust" est toujours présent
		// et que "Maîtriser Java" n'a pas été ajouté
		assert!(tasks.iter().any(|task| task.title == "Apprendre Rust"));
		assert!(!tasks.iter().any(|task| task.title == "Maîtriser Java"));
	}

	#[test]
	fn rename_task_accepts_same_title() {
		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter "Apprendre Rust"
		let result = add_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, AddTaskResult::Added);

		// 3. Renommer "Apprendre Rust"en "Apprendre Rust"
		let result = rename_task(
			&mut tasks, 
			"Apprendre Rust", 
			"Apprendre Rust");
		
		// 4. Vérifier que le résultat est RenameTaskResult::Renamed
		assert_eq!(result, RenameTaskResult::Renamed);

		// 5. Vérifier que le vecteur contient toujours une seule tâche
		assert_eq!(tasks.len(), 1);

		// 6. Vérifier que son titre est toujours "Apprendre Rust"
		assert_eq!(tasks[0].title, "Apprendre Rust");
	}

	#[test]
	fn rename_task_preserves_completed_status() {
		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter une tâche nommée "Apprendre Rust"
		let result = add_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, AddTaskResult::Added);

		// 3. Appeler complete_task() pour la marquer comme terminée
		let result = complete_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, CompleteTaskResult::Completed);

		// 4. Renommer la tâche en "Maîtriser Rust"
		let result = rename_task(
			&mut tasks, 
			"Apprendre Rust", 
			"Maîtriser Rust");

		// 5. Vérifier que le résultat est RenameTaskResult::Renamed
		assert_eq!(result, RenameTaskResult::Renamed);

		// 6. Vérifier que le titre de la tâche est "Maîtriser Rust"
		assert_eq!(tasks[0].title, "Maîtriser Rust");

		// 7. Vérifier que tasks[0].completed vaut toujours true
		assert!(tasks[0].completed);
	}

	#[test]
	fn rename_task_rejects_empty_title() {
		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter une tâche intitulée "Apprendre Rust"
		let result = add_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, AddTaskResult::Added);

		// 3. Tenter de renommer cette tâche avec ""
		let result = rename_task(
			&mut tasks, 
			"Apprendre Rust", 
			"");
		
		// 4. Vérifie que le renommage est refusé
		assert_eq!(result, RenameTaskResult::InvalidTitle);

		// 5. Vérifier que la tâche originale existe toujours avec son 
		// ancien titre
		assert_eq!(tasks[0].title, "Apprendre Rust");

		// 6. Vérifier que le nombre de tâches n'a pas changé
		assert_eq!(tasks.len(), 1);
	}

	#[test]
	fn rename_task_rejects_whitespace_only_title() {
		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter une tâche nommée "Apprendre Rust"
		let result = add_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, AddTaskResult::Added);

		// 3. Essayer de la renommer avec "   " (trois espaces)
		let result = rename_task(
			&mut tasks, 
			"Apprendre Rust", 
			"   ");
		
		// 4. Vérifier que le résultat est RenameTaskResult::InvalidTitle
		assert_eq!(result, RenameTaskResult::InvalidTitle);

		// 5. Vérifier que le titre original est toujours "Apprendre Rust"
		assert_eq!(tasks[0].title, "Apprendre Rust");

		// 6. Vérifier que le nombre de tâches est toujours égal à 1
		assert_eq!(tasks.len(), 1);
	}

	#[test]
	fn add_task_rejects_empty_title() {
		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Appeler add_task() avec ""
		let result = add_task(&mut tasks, "");

		// 3. Vérifier que le résultat indique un titre invalide
		assert_eq!(result, AddTaskResult::InvalidTitle);

		// 4. Vérifier que le vecteur est toujours vide
		assert!(tasks.is_empty());
	}

	#[test]
	fn add_task_rejects_whitespace_only_title() {
		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter une tâche avec "   " (trois espaces)
		let result = add_task(&mut tasks, "   ");

		// 3. Vérifier que le résultat est AddTaskResult::InvalidTitle
		assert_eq!(result, AddTaskResult::InvalidTitle);

		// 4. Vérifier que le vecteur est toujours vide
		assert!(tasks.is_empty());
	}

	#[test]
	fn add_task_trims_title_whitespace() {
		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Appeler add_task() avec " Apprendre Rust "
		let result = add_task(&mut tasks, " Apprendre Rust ");

		// 3. Vérifier que le résultat est AddTaskResult::Added
		assert_eq!(result, AddTaskResult::Added);

		// 4. Vérifier que le titre stocké est exactement "Apprendre Rust"
		assert_eq!(tasks[0].title, "Apprendre Rust");
	}

	#[test]
	fn rename_task_trims_title_whitespace() {

		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter une tâche appelée "Apprendre Rust"
		let result = add_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, AddTaskResult::Added);

		// 3. Renommer-la en utilisant " Maîtriser Rust "
		let result = rename_task(
			&mut tasks, 
			"Apprendre Rust", 
			" Maîtriser Rust ");
		
		// 4. Vérifier que le résultat est RenameTaskResult::Renamed
		assert_eq!(result, RenameTaskResult::Renamed);

		// 5. Vérifier que le titre stocké est exactement "Maîtriser Rust"
		assert_eq!(tasks[0].title, "Maîtriser Rust");
	}

	#[test]
	fn add_task_rejects_duplicate_title_with_whitespace() {
		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter une première tâche
		let result = add_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, AddTaskResult::Added);

		// 3. Ajouter une seconde tâche avec "  Apprendre Rust  "
		// renvoie AddTaskResult::AlreadyExists
		let result = add_task(&mut tasks, "  Apprendre Rust  ");
		assert_eq!(result, AddTaskResult::AlreadyExists);

		// 4. Vérifier que la liste contient toujours une seule tâche
		assert!(tasks.len() == 1);
	}

	#[test]
	fn rename_task_rejects_existing_title_with_whitespace() {
		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter deux tâches
		let result = add_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, AddTaskResult::Added);
		let result = add_task(&mut tasks, "Apprendre Java");
		assert_eq!(result, AddTaskResult::Added);

		// 3. Renommer "Apprendre Rust" en "  Apprendre Java  " renvoie
		// AlreadyExistsForRename
		let result = rename_task(
			&mut tasks, 
			"Apprendre Rust", 
			"  Apprendre Java  ");
		assert_eq!(result, RenameTaskResult::AlreadyExists);

		// 4. Vérifier que les titres d'origine sont inchangés
		// et que la liste contient toujours deux tâches.
		assert!(tasks[0].title == "Apprendre Rust");
		assert!(tasks[1].title == "Apprendre Java");
		assert!(tasks.len() == 2);
	}

	#[test]
	fn rename_task_accepts_same_title_with_whitespace() {
		// 1. Créer un vecteur vide
		let mut tasks: Vec<Task> = Vec::new();

		// 2. Ajouter la tâche "Apprendre Rust" renvoie 
		// AddTaskResult::Added
		let result = add_task(&mut tasks, "Apprendre Rust");
		assert_eq!(result, AddTaskResult::Added);

		// 3. Renommer cette tâche en "  Apprendre Rust  " renvoie
		// RenameTaskResult::Renamed
		let result = rename_task(
			&mut tasks, 
			"Apprendre Rust", 
			"  Apprendre Rust  ");
		assert_eq!(result, RenameTaskResult::Renamed);
			
		// 4. Vérifier que le titre rest "Apprendre Rust"
		// et que le vecteur contient toujours une seule tâche.
		assert_eq!(tasks[0].title, "Apprendre Rust");
		assert_eq!(tasks.len(), 1);
	}
}