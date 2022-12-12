<template>
	<p>Font Boxes</p>
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