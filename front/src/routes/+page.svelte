<script lang="ts">
	import HackerText from "$lib/components/HackerText.svelte"
</script>

<svelte:head>
    <title>The Maze</title>
</svelte:head>

<div class="w-full h-full flex flex-col justify-center items-center">
    <div class="flex justify-center w-full">
        <span class="text-6xl text-center">
            Are you ready to beat
            <br />
            <span class="text-primary font-extrabold">
                <HackerText text="the maze" />
            </span>
            ?
        </span>
    </div>
    
    <div class="mt-28">
        <a class="btn btn-wide btn-primary btn-lg" href="/game">Play now !</a>
    </div>
</div>
