import hole_right from './hole_right.svg';
import hole_left from './hole_left.svg';
import hole_up from './hole_up.svg';
import hole_bottom from './hole_bottom.svg';

import four_walls from './4_walls.svg';

import two_walls_bottom_left from './2_walls_bottom_left.svg';
import two_walls_right_bottom from './2_walls_right_bottom.svg';
import two_walls_right_left from './2_walls_right_left.svg';
import two_walls_up_right from './2_walls_up_right.svg';
import two_walls_up_bottom from './2_walls_up_bottom.svg';
import two_walls_up_left from './2_walls_up_left.svg';

import wall_bottom from './wall_bottom.svg';
import wall_left from './wall_left.svg';
import wall_right from './wall_right.svg';
import wall_up from './wall_up.svg';

import no_wall from './no_wall.png';

export default function compute_walls(walls: [boolean, boolean, boolean, boolean]) {
	if (walls[0] && walls[1] && walls[2] && walls[3]) {
		return four_walls;
	}

	if (walls[0] && walls[1] && walls[2]) {
		return hole_left;
	}

	if (walls[0] && walls[1] && walls[3]) {
		return hole_bottom;
	}

	if (walls[0] && walls[2] && walls[3]) {
		return hole_right;
	}

	if (walls[1] && walls[2] && walls[3]) {
		return hole_up;
	}

	if (walls[0] && walls[1]) {
		return two_walls_up_right;
	}

	if (walls[0] && walls[2]) {
		return two_walls_up_bottom;
	}

	if (walls[0] && walls[3]) {
		return two_walls_up_left;
	}

	if (walls[1] && walls[2]) {
		return two_walls_right_bottom;
	}

	if (walls[1] && walls[3]) {
		return two_walls_right_left;
	}

	if (walls[2] && walls[3]) {
		return two_walls_bottom_left;
	}

	if (walls[0]) {
		return wall_up;
	}

	if (walls[1]) {
		return wall_right;
	}

	if (walls[2]) {
		return wall_bottom;
	}

	if (walls[3]) {
		return wall_left;
	}

	return no_wall;
}
