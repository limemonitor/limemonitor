<script lang="ts">
	import { page } from '$app/stores';
	import Icon from '@iconify/svelte';

	const buttons = [
		{
			name: 'Dashboard',
			path: '/',
			icon: 'akar-icons:dashboard'
		},
		{
			name: 'Network',
			path: '/network',
			icon: 'lucide:network'
		},
		{
			name: 'Firewall',
			path: '/firewall',
			icon: 'carbon:firewall'
		},
		{
			name: 'Alerts',
			path: '/alerts',
			icon: 'ant-design:alert-outlined'
		},
		{
			name: 'Settings',
			path: '/settings',
			icon: 'solar:settings-outline'
		}
	];
</script>

<div
	class="no-select fixed bottom-0 left-0 top-8 min-w-[4.5rem] max-w-[4.5rem] border-r border-border bg-[rgb(var(--menubar))]"
>
	<div
		class="absolute left-0 top-0 h-[4.5rem] w-px rounded-r-md bg-primary shadow-[0_0_20px_5px_rgba(var(--primary),0.5)] transition duration-500"
		style={`transform: translateY(calc(100%*${buttons.findIndex(
			(btn) => btn.path === $page.url.pathname
		)}));`}
	/>

	{#each buttons as btn}
		<a
			href={btn.path}
			class={`relative block h-[4.5rem] w-[4.5rem] p-6 transition duration-300 hover:ease-out ${
				$page.url.pathname === btn.path
					? 'text-color drop-shadow-[0_0_10px_rgb(var(--primary))]'
					: 'text-color-faint hover:text-color'
			}`}
		>
			<Icon icon={btn.icon} class="-mt-2.5 h-6 w-6" />

			<span class="absolute bottom-2.5 left-1/2 -translate-x-1/2 text-xs">{btn.name}</span>
		</a>
	{/each}
</div>

<div class="relative h-full min-w-[4.5rem] max-w-[4.5rem]"></div>
