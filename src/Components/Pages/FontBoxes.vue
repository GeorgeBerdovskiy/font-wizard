<template>
	<div>
		<h1 class="margin-bottom-half">Font Boxes</h1>

		<div class="flex-card-grid">
			<div class="card">
				<p>Modern Fonts</p>
			</div>

			<div class="card">
				<p>Classic Fonts</p>
			</div>
		</div>

		<button @click="createDataFolder">Create new directory.</button>
	</div>
</template>

<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { BaseDirectory, createDir, writeTextFile, readTextFile, exists } from "@tauri-apps/api/fs";
	import { appDataDir } from '@tauri-apps/api/path';

	export default {
		data() {
			return {
				boxes: ""
			}
		},

		methods: {
			async createDataFolder() {
				const appDataDirPath = await appDataDir();
				let directoryExists = await exists(appDataDirPath + "data/font-boxes.json");
				
				if (directoryExists) {
					console.log("DEBUG - Data directory with font boxes JSON file already exists.")
					return;
				}

				try {
					await createDir("data", {
						dir: BaseDirectory.App,
						recursive: true,
					});

					await writeTextFile({ path: "data/font-boxes.json", contents: '[ { "name": "Classic Fonts" }, { "name": "Modern Fonts" } ]' }, { dir: BaseDirectory.App });
				} catch (error) {
					console.error(error);
				}
			},

			async collectBoxes() {
				try {
					const data = await readTextFile("data/font-boxes.json", { dir: BaseDirectory.App });
					this.boxes = JSON.parse(data);
				} catch (error) {
					console.error(error);
				}
			}
		},

		async created() {
			await this.collectBoxes();
		}
	}
</script>

<style scoped>
	.flex-card-grid {
		display: flex;
		margin-right: calc(-0.5 * var(--space));
	}

	.card {
		padding: 32px;
		background-color: rgb(var(--evergreen));
		border-radius: 10px;

		margin-right: calc(0.5 * var(--space));

		color: white;
		cursor: pointer;
	}
</style>