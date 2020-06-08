shader_type spatial;
render_mode depth_draw_always;

uniform vec3 arg_color = vec3(1, 1, 1);
uniform float arg_alpha = 1.0;
uniform mat4 arg_texcoord_transform = mat4(1);
uniform sampler2D arg_texture;
uniform sampler2D arg_reflection;
uniform bool has_texture = false;
uniform bool has_reflection = false;

void fragment() {
	mat3 realmat = mat3(
		arg_texcoord_transform[0].xyz,
		arg_texcoord_transform[1].xyz,
		arg_texcoord_transform[2].xyz
	);
	vec3 uvpos = vec3(UV.x, UV.y, 1.0);
	uvpos = uvpos * realmat;
	vec4 col1;
	if (has_texture) {
		col1 = texture(arg_texture, uvpos.xy);
	} else {
		col1 = vec4(1, 1, 1, 1);
	}
	vec4 col2;
	if (has_reflection) {
		col2 = textureProj(arg_reflection, reflect(VIEW, normalize(NORMAL)));
	} else {
		col2 = vec4(0, 0, 0, 0);
	}
	vec4 outcol = col1 * vec4(arg_color, arg_alpha);
	outcol.rgb += col2.rgb * col2.a;
	ALBEDO = outcol.rgb;
	ALPHA = outcol.a;
}