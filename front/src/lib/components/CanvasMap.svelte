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

	let canvas: HTMLCanvasElement;
	let ctx: CanvasRenderingContext2D | null;
	const PIXEL_SIZE = 15;

	onMount(() => {
		canvas = document.getElementById('map') as HTMLCanvasElement;
		ctx = canvas.getContext('2d');

		if (!ctx) return;
	});

	function set_pixel(pos: { x: number; y: number }, color: string, ctx: CanvasRenderingContext2D) {
		ctx.fillStyle = color;
		ctx.fillRect(pos.x * PIXEL_SIZE, pos.y * PIXEL_SIZE, PIXEL_SIZE, PIXEL_SIZE);
	}

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

<canvas id="map" />
