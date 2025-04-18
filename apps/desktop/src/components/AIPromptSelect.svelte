<script lang="ts">
	import { PromptService } from '$lib/ai/promptService';
	import { Project } from '$lib/project/project';
	import { getContext } from '@gitbutler/shared/context';
	import Select from '@gitbutler/ui/select/Select.svelte';
	import SelectItem from '@gitbutler/ui/select/SelectItem.svelte';
	import { onMount } from 'svelte';
	import type { Prompts, UserPrompt } from '$lib/ai/types';
	import type { Persisted } from '@gitbutler/shared/persisted';

	interface Props {
		promptUse: 'commits' | 'branches';
	}

	const { promptUse }: Props = $props();

	const project = getContext(Project);
	const promptService = getContext(PromptService);

	let prompts: Prompts;
	let selectedPromptId = $state<Persisted<string | undefined>>();

	if (promptUse === 'commits') {
		prompts = promptService.commitPrompts;
		selectedPromptId = promptService.selectedCommitPromptId(project.id);
	} else {
		prompts = promptService.branchPrompts;
		selectedPromptId = promptService.selectedBranchPromptId(project.id);
	}

	let userPrompts = prompts.userPrompts;
	let allPrompts: UserPrompt[] = $state([]);

	const defaultId = crypto.randomUUID();

	function setAllPrompts(userPrompts: UserPrompt[]) {
		allPrompts = [
			{ name: 'Default Prompt', id: defaultId, prompt: prompts.defaultPrompt },
			...userPrompts
		];
	}

	onMount(() => {
		setAllPrompts($userPrompts);
	});

	$effect(() => {
		if (!$selectedPromptId || !promptService.findPrompt(allPrompts, $selectedPromptId)) {
			$selectedPromptId = defaultId;
		}
	});
</script>

<Select
	value={$selectedPromptId}
	options={allPrompts.map((p) => ({ label: p.name, value: p.id }))}
	label={promptUse === 'commits' ? 'Commit message' : 'Branch name'}
	wide={true}
	searchable
	disabled={allPrompts.length === 1}
	onselect={(value) => {
		$selectedPromptId = value;
	}}
>
	{#snippet itemSnippet({ item, highlighted })}
		<SelectItem selected={item.value === $selectedPromptId} {highlighted}>
			{item.label}
		</SelectItem>
	{/snippet}
</Select>
