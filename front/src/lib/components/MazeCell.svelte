<script lang="ts">
    import compute_walls from "$lib/assets";
    import { onMount, afterUpdate } from "svelte";

	export let walls: [boolean, boolean, boolean, boolean];
    export let animate: [boolean, boolean, boolean, boolean];

    let src: string;
    
    onMount(() => {
        src = compute_walls(walls);
    });

    afterUpdate(() => {
        src = compute_walls(walls);
    });
</script>

<div class="flex flex-row"
    class:animate_u={animate[0]}
    class:animate_r={animate[1]}
    class:animate_b={animate[2]}
    class:animate_l={animate[3]}>
    <img src="{src}" alt="cell" />
</div>

<style lang="scss">
	@keyframes r_shake {
		0% {
			transform: translateX(0);
		}
		25% {
			transform: translateX(10px);
		}
		50% {
			transform: translateX(-10px);
		}
		75% {
			transform: translateX(10px);
		}
		100% {
			transform: translateX(0);
		}
	}

    @keyframes l_shake {
		0% {
			transform: translateX(0);
		}
		25% {
			transform: translateX(-10px);
		}
		50% {
			transform: translateX(10px);
		}
		75% {
			transform: translateX(-10px);
		}
		100% {
			transform: translateX(0);
		}
	}

    @keyframes u_shake {
		0% {
			transform: translateY(0);
		}
		25% {
			transform: translateY(10px);
		}
		50% {
			transform: translateY(-10px);
		}
		75% {
			transform: translateY(10px);
		}
		100% {
			transform: translateY(0);
		}
	}

    @keyframes b_shake {
		0% {
			transform: translateY(0);
		}
		25% {
			transform: translateY(-10px);
		}
		50% {
			transform: translateY(10px);
		}
		75% {
			transform: translateY(-10px);
		}
		100% {
			transform: translateY(0);
		}
	}

    .animate_b {
        animation: b_shake 0.2s ease-in-out;
    }

    .animate_l {
        animation: l_shake 0.2s ease-in-out;
    }

    .animate_r {
        animation: r_shake 0.2s ease-in-out;
    }

    .animate_u {
        animation: u_shake 0.2s ease-in-out;
    }
</style>
