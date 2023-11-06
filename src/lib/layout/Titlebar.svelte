<script lang="ts">
	import Icon from '@iconify/svelte';
	import { appWindow } from '@tauri-apps/api/window';
	let maximized = false;

	onresize = async () => {
		maximized = await appWindow.isMaximized();
	};
</script>

<div class="relative h-10"></div>
<div
	data-tauri-drag-region
	class="no-select fixed left-0 right-0 top-0 flex h-8 justify-between border-b border-border bg-[rgb(var(--titlebar))]"
>
	<div class="flex">
		<img src="/icon.svg" alt="App Icons" class="m-1 h-6 w-6" />
		<p class="py-1.5 pl-px text-sm">LimeMonitor</p>
	</div>
	<div class="flex">
		<button
			class="h-8 w-10 cursor-default px-3 py-2 transition duration-200 hover:bg-white/20 hover:ease-out active:bg-white/20 active:transition-none"
			on:click={() => {
				appWindow.minimize();
			}}
		>
			<Icon icon="fluent:minimize-12-filled" class="h-4 w-4" />
		</button>
		<button
			class="h-8 w-10 cursor-default px-3 py-2 transition duration-200 hover:bg-white/20 hover:ease-out active:bg-white/20 active:transition-none"
			on:click={() => {
				appWindow.toggleMaximize();
			}}
		>
			<Icon icon={maximized ? 'bx:windows' : 'fluent:maximize-16-filled'} class="h-4 w-4" />
		</button>
		<button
			class="h-8 w-10 cursor-default px-3 py-2 transition duration-200 hover:bg-red-600 hover:ease-out active:bg-red-800 active:transition-none"
			on:click={() => {
				appWindow.close();
			}}
		>
			<Icon icon="iconamoon:close-bold" class="h-4 w-4" />
		</button>
	</div>
</div>
