import type { PageLoad } from './$types';
import { invoke } from '@tauri-apps/api/core';

export const load = (async () => {
	type Projects = {
		id: number;
		name: string;
		description: string;
		root_folder_path: string;
		github_link: string;
		read_me_link: string;
		readme_content?: string;
	};

	// pub struct Project {
	//     pub name: String,
	//     pub root_folder_path: String,
	//     pub description: Option<String>,
	//     pub github_link: Option<String>,
	//     pub read_me_link: Option<String>,
	// }

	const projects: Projects[] = await invoke('scan_project_folder', {
		path: 'C:\\Users\\kotsu\\Desktop\\Production'
	});

	// let projects: Projects[] = [
	// 	{
	// 		id: 1,
	// 		title: 'UR Rusting',
	// 		description:
	// 			'A SvelteKit application that generates project documentation from GitHub repositories.',
	// 		local_link: '/Users/bzar/Documents/Production Projects/project-documenter',
	// 		git_link: 'https://github.com/bannerblack/ur_rusting',
	// 		readme_link: '/Users/bzar/Documents/Production Projects/project-documenter/README.md',
	// 		readme_content: '/Users/bzar/Documents/Production Projects/project-documenter/README.md',
	// 		related_projects: [2, 3]
	// 	},
	// 	{
	// 		id: 2,
	// 		title: 'Task Manager',
	// 		description: 'A web-based task management tool built with SvelteKit and TypeScript.',
	// 		local_link: '/Users/bzar/Documents/Production Projects/project-documenter',
	// 		git_link: '',
	// 		readme_link: '/Users/bzar/Documents/Production Projects/project-documenter/README.md',
	// 		readme_content: '',
	// 		related_projects: [1]
	// 	},
	// 	{
	// 		id: 3,
	// 		title: 'Portfolio Website',
	// 		description:
	// 			'A personal portfolio website showcasing projects and skills, developed using SvelteKit.',
	// 		local_link: '/Users/bzar/Documents/Production Projects/project-documenter   ',
	// 		git_link: '',
	// 		readme_link: '/Users/bzar/Documents/Production Projects/project-documenter/README.md',
	// 		related_projects: [1],
	// 		readme_content: ''
	// 	}
	// ];
	return { projects };
}) satisfies PageLoad;
