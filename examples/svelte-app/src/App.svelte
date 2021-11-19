<script lang="ts">
	import { move_window, Position } from 'tauri-plugin-positioner-api'

	function _top_left() {
		move_window(Position.TopLeft)
	}

	function _top_right() {
		move_window(Position.TopRight)
	}

	function _bottom_left() {
		move_window(Position.BottomLeft)
	}

	function _bottom_right() {
		move_window(Position.BottomRight)
	}

	function _top_center() {
		move_window(Position.TopCenter)
	}

	function _bottom_center() {
		move_window(Position.BottomCenter)
	}

	function _left_center() {
		move_window(Position.LeftCenter)
	}

	function _right_center() {
		move_window(Position.RightCenter)
	}

	function _center() {
		move_window(Position.Center)
	}
</script>

<div>
	<button on:click="{_top_left}">TopLeft</button>
	<button on:click="{_top_right}">TopRight</button>
	<button on:click="{_bottom_left}">BottomLeft</button>
	<button on:click="{_bottom_right}">BottomRight</button>
	<button on:click="{_top_center}">TopCenter</button>
	<button on:click="{_bottom_center}">BottomCenter</button>
	<button on:click="{_left_center}">LeftCenter</button>
	<button on:click="{_right_center}">RightCenter</button>
	<button on:click="{_center}">Center</button>
</div>
