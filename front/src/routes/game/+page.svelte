<script lang="ts">
	import { onMount } from 'svelte';
	import { call } from '$lib/api';
	import type { PageData } from './$types';
	import Canvas from '$lib/components/canvas.svelte';

	export let data: PageData;

	type walls = [boolean, boolean, boolean, boolean];

	let canvas: Canvas;
	let loading = false;
	let logged = false;
	let name = '';
	let walls: walls = [false, false, false, false];
	let cells: any[] = [];
	let animate: walls = [false, false, false, false];
	let win = false;

	let eventFunc = (e: KeyboardEvent) => {
		switch (e.key) {
			case 'ArrowUp' || 'z' || 'Z' || 'w' || 'W':
				move('up');
				break;
			case 'ArrowDown' || 's' || 'S' || 'x' || 'X':
				move('down');
				break;
			case 'ArrowLeft' || 'q' || 'Q' || 'a' || 'A':
				move('left');
				break;
			case 'ArrowRight' || 'd' || 'D' || 'e' || 'E':
				move('right');
				break;
			default:
				break;
		}
	};

	onMount(() => {
		cells = [
			[
				[false, false, false, false],
				[false, false, false, false],
				[false, false, false, false]
			],
			[
				[false, false, false, false],
				[false, false, false, false],
				[false, false, false, false]
			],
			[
				[false, false, false, false],
				[false, false, false, false],
				[false, false, false, false]
			]
		];

		if (data.cookie) {
			let resp = call('/client/', null, 'GET', null, null);

			resp.then((res) => {
				if (res.status === 200) {
					logged = true;

					res
						.json()
						.then((json) => {
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
							} else {
								update(json.client);
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
		// Clear the canvas
		canvas.clear();

		walls = client.curr_cell as [boolean, boolean, boolean, boolean];
		let pos = client.pos as { x: number; y: number };
		pos = { x: pos.y, y: pos.x };

		let i = 0;
		let j = 0;

		client.neighbors.forEach((n: walls) => {
			if (n == null) {
				cells[i][j] = [false, false, false, false];
			} else {
				cells[i][j] = n as [boolean, boolean, boolean, boolean];
			}

			canvas.draw({ x: j, y: i }, cells[i][j], 'white');

			j++;

			if (j === 3) {
				j = 0;
				i++;
			}
		});
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
					res.json().then((json) => {
						update(json.client);
					});
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
		<div class="flex flex-col justify-center items-center w-full h-full">
			<Canvas bind:this={canvas} animate={animate} />
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
