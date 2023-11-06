<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	let data = '';

	const getData = async () => {
		data = await invoke('get_dashboard');
	};

	getData();
</script>

<div class="p-10">
	<h1 class="text-4xl font-semibold">Dashboard</h1>
	<p class="mt-5 text-lg text-color-secondary">{JSON.stringify(data)}</p>
</div>
