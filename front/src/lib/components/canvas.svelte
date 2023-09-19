<script lang="ts">
	import { onMount } from 'svelte';

	export function draw(
		pos: { x: number; y: number },
		walls: [boolean, boolean, boolean, boolean],
		walls_color: string
	) {
		if (!ctx) return;
		draw_walls(pos, walls, walls_color, ctx);
	}

    export function clear() {
        if (!ctx) return;
        ctx.clearRect(0, 0, canvas.width, canvas.height);
    }

    export let animate: [boolean, boolean, boolean, boolean];

	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D | null;
	const PIXEL_SIZE = 100;
    const width = 3 * PIXEL_SIZE;
    const height = 3 * PIXEL_SIZE;

	onMount(() => {
		canvas = document.getElementById('map') as HTMLCanvasElement;
		ctx = canvas.getContext('2d');

		if (!ctx) return;
	});

	function draw_walls(
		pos: { x: number; y: number },
		walls: [boolean, boolean, boolean, boolean],
		walls_color: string,
		ctx: CanvasRenderingContext2D
	) {
		ctx.beginPath();
		ctx.moveTo(pos.x * PIXEL_SIZE, pos.y * PIXEL_SIZE);

		if (walls[0]) ctx.lineTo(pos.x * PIXEL_SIZE + PIXEL_SIZE, pos.y * PIXEL_SIZE);
		ctx.moveTo(pos.x * PIXEL_SIZE + PIXEL_SIZE, pos.y * PIXEL_SIZE);

		if (walls[1]) ctx.lineTo(pos.x * PIXEL_SIZE + PIXEL_SIZE, pos.y * PIXEL_SIZE + PIXEL_SIZE);
		ctx.moveTo(pos.x * PIXEL_SIZE + PIXEL_SIZE, pos.y * PIXEL_SIZE + PIXEL_SIZE);

		if (walls[2]) ctx.lineTo(pos.x * PIXEL_SIZE, pos.y * PIXEL_SIZE + PIXEL_SIZE);
		ctx.moveTo(pos.x * PIXEL_SIZE, pos.y * PIXEL_SIZE + PIXEL_SIZE);

		if (walls[3]) ctx.lineTo(pos.x * PIXEL_SIZE, pos.y * PIXEL_SIZE);
		ctx.moveTo(pos.x * PIXEL_SIZE, pos.y * PIXEL_SIZE);

		ctx.strokeStyle = walls_color;
		ctx.closePath();
		ctx.stroke();
	}
</script>

<canvas id="map" width={width} height={height} class="absolute z-10" 
    class:animate_u={animate[0]}
    class:animate_r={animate[1]}
    class:animate_b={animate[2]}
    class:animate_l={animate[3]} />
<span class="absolute z-20 text-white">x</span>

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
