<script lang="ts">
	import Menubar from '@lib/layout/Menubar.svelte';
	import Sidebar from '@lib/layout/Sidebar.svelte';
	import Titlebar from '@lib/layout/Titlebar.svelte';

	import '@lib/styles/tailwind.css';
	import '../app.css';
</script>

<body>
	<Titlebar />
	<div class="flex">
		<Menubar />
		<Sidebar />

		<div class="w-full">
			<slot />
		</div>
	</div>
</body>
