export type Projects = {
		id: number;
		name: string;
		parent: string;
		description: string;
		root_folder_path: string;
		github_link: string;
		read_me_link: string;
		readme_content?: string;
		priority: number;
		tags: string[];
	};