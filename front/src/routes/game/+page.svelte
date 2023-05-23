<script lang="ts">
	import { onMount } from 'svelte';
	import { call } from '$lib/api';
	import type { PageData } from './$types';
	import MazeCell from '$lib/components/MazeCell.svelte';

	export let data: PageData;

	let loading = false;
	let logged = false;
	let name = '';
	let walls: [boolean, boolean, boolean, boolean] = [false, false, false, false];
    let animate: [boolean, boolean, boolean, boolean]  = [false, false, false, false];
    
    let eventFunc = (e) => {
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

					res.json().then((json) => {
						name = json.client.name;
						walls = json.client.curr_cell as [boolean, boolean, boolean, boolean];

						if (json.client.is_playing === 'false') {
							call('/client/new_game', null, 'POST', null, null).then((res) => {
								if (res.status === 200) {
									res.json().then((json) => {
										walls = json.client.curr_cell as [boolean, boolean, boolean, boolean];
									});
								} else {
									alert('An error occured');
								}
							});
						}
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
        }
	});

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
					call('/client/new_game', null, 'POST', null, null).then((res) => {
						if (res.status === 200) {
							res.json().then((json) => {
								walls = json.client.curr_cell as [boolean, boolean, boolean, boolean];
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

    function move(direction: string) {
        loading = true;

        let data = new FormData();
        data.append('direction', direction);

        let resp = call('/client/move', null, 'POST', data, null);

        resp.then((res) => {
            if (res.status === 200) {
                res.json().then((json) => {
                    if (json.movement === 'true') {
                        walls = json.client.curr_cell as [boolean, boolean, boolean, boolean];
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
                        };

                        setTimeout(() => {
                            animate = [false, false, false, false];
                        }, 220);
                    }
                })
            }
        })
    }
</script>

{#if logged}
	<div class="flex flex-col w-full h-full justify-center items-center">
		<MazeCell {walls} {animate} />
	</div>
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
			<button
				class="btn btn-primary mt-10"
				class:loading
				on:click={() => {
					login();
				}}
				>Validate
			</button>
		</div>
	</div>
{/if}
