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
	</div>
</template>

<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	export default {
		data() {
			return {
				boxes: ""
			}
		},

		methods: {
			async collectBoxes() {
				this.boxes = await invoke("collect_boxes");
				console.log(this.boxes);
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