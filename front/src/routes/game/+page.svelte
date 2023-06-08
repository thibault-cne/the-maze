<script lang="ts">
	import { onMount } from 'svelte';
	import { call } from '$lib/api';
	import type { PageData } from './$types';
	import MazeCell from '$lib/components/MazeCell.svelte';
	import CanvasMap from '$lib/components/CanvasMap.svelte';

	export let data: PageData;

	let loading = false;
	let logged = false;
	let name = '';
	let walls: [boolean, boolean, boolean, boolean] = [false, false, false, false];
	let animate: [boolean, boolean, boolean, boolean] = [false, false, false, false];
	let win = false;

	let draw: (
		pos: {
			x: number;
			y: number;
		},
		walls: [boolean, boolean, boolean, boolean],
		walls_color: string
	) => void;

	let eventFunc = (e: KeyboardEvent) => {
		switch (e.key) {
			case 'ArrowUp':
				move('up');
				break;
			case 'ArrowDown':
				move('down');
				break;
			case 'ArrowLeft':
				move('left');
				break;
			case 'ArrowRight':
				move('right');
				break;
			default:
				break;
		}
	};

	onMount(() => {
		if (data.cookie) {
			let resp = call('/client/', null, 'GET', null, null);

			resp.then((res) => {
				if (res.status === 200) {
					logged = true;

					res
						.json()
						.then((json) => {
							name = json.client.name;
							walls = json.client.curr_cell as [boolean, boolean, boolean, boolean];

							if (json.client.is_playing === 'false') {
								call('/client/play', null, 'POST', null, null).then((res) => {
									if (res.status === 200) {
										res
											.json()
											.then((json) => {
												update(json.client);
											})
											.catch((e) => {
												console.log(e);
											});
									} else {
										alert('An error occured');
									}
								});
							}
						})
						.catch((e) => {
							console.log(e);
						});
				} else {
					alert('An error occured');
				}
			});
		}

		// Handle keypress
		window.addEventListener('keydown', eventFunc);

		return () => {
			window.removeEventListener('keydown', eventFunc);
		};
	});

	function update(client: any) {
		walls = client.curr_cell as [boolean, boolean, boolean, boolean];
		let pos = client.pos as { x: number; y: number };
		pos = { x: pos.y, y: pos.x };
		console.log(pos);
		draw(pos, walls, '#ffffff');
	}

	function login() {
		loading = true;

		setTimeout(() => {
			if (name === '') {
				alert('Please enter your name');
				loading = false;
				return;
			}

			let body = new FormData();
			body.append('username', name);

			let resp = call('/client/login', null, 'POST', body, null);

			resp.then((res) => {
				if (res.status === 200) {
					logged = true;

					// TODO: check and handle errors
					call('/client/play', null, 'POST', null, null).then((res) => {
						if (res.status === 200) {
							res.json().then((json) => {
								update(json.client);
							});
						} else {
							alert('An error occured');
						}
					});
				} else {
					alert('An error occured');
				}
			});

			loading = false;
		}, 1000);
	}

	function replay() {
		loading = true;

		setTimeout(() => {
			let resp = call('/client/play', null, 'POST', null, null);

			resp.then((res) => {
				if (res.status === 200) {
					win = false;
				} else {
					alert('An error occured');
				}
			});

			loading = false;
		}, 1000);
	}

	function move(direction: string) {
		if (loading || !logged) return;

		loading = true;

		let data = new FormData();
		data.append('direction', direction);

		let resp = call('/client/move', null, 'POST', data, null);

		resp.then((res) => {
			if (res.status === 200) {
				res.json().then((json) => {
					if (json.status === 'win') {
						win = true;
						return;
					}
					if (json.movement === 'true') {
						update(json.client);
					} else {
						switch (json.direction) {
							case 'up':
								animate[0] = true;
								break;
							case 'down':
								animate[2] = true;
								break;
							case 'left':
								animate[3] = true;
								break;
							case 'right':
								animate[1] = true;
								break;
							default:
								break;
						}

						setTimeout(() => {
							animate = [false, false, false, false];
						}, 220);
					}
				});
			}

			loading = false;
		});
	}
</script>

{#if logged}
	{#if win}
		<div class="flex flex-col h-full justify-center items-center w-full">
			<div class="flex flex-col justify-center items-center">
				<span class="p-2 font-bold">You won !</span>
				<button class="btn btn-primary mt-10" on:click={replay} class:loading> Play again </button>
			</div>
		</div>
	{:else}
		<div class="grid w-full h-full items-center">
			<div class="flex justify-center items-end h-full">
				<MazeCell {walls} {animate} />
			</div>
			<div class="flex w-full h-full justify-end items-end">
				<div class="pb-5">
					<CanvasMap bind:draw />
				</div>
			</div>
		</div>
	{/if}
{:else}
	<div class="flex flex-col h-full justify-center items-center w-full">
		<div class="flex flex-col">
			<span class="p-2 font-normal">Enter your name :</span>
			<input
				bind:value={name}
				type="text"
				placeholder="Enter your name"
				class="input input-bordered input-primary w-full max-w-xs focus:outline-offset-0"
			/>
			<button class="btn btn-primary mt-10" class:loading on:click={login}>Validate </button>
		</div>
	</div>
{/if}
