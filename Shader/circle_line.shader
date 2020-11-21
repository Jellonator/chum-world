shader_type spatial;
render_mode unshaded, cull_disabled,depth_draw_opaque;

void fragment() {
	ALPHA_SCISSOR = 0.5;
	float dis = length(UV - vec2(0.5)) * 2.0;
	if (dis > 1.0 || dis < 0.9) {
		ALPHA = 0.0;
	} else {
		ALPHA = 1.0;
	}
}