<script lang="ts">
	import { page } from '$app/state';
	import BranchesView from '$components/v3/BranchesView.svelte';

	const projectId = $derived(page.params.projectId!);
</script>

<BranchesView {projectId} />

<style lang="postcss">
</style>
