shader_type spatial;
render_mode depth_draw_alpha_prepass, cull_disabled, skip_vertex_transform;

uniform vec3 arg_color = vec3(1, 1, 1);
uniform float arg_alpha = 1.0;
uniform mat4 arg_texcoord_transform = mat4(1);
uniform sampler2D arg_texture;
uniform sampler2D arg_reflection;
uniform bool has_texture = false;
uniform bool has_reflection = false;

void vertex() {
	// Small hack: special, mesh-specific rendering information is stored in UV2.
	// To ensure that the UV2 is special, values in [0, 1] are ignored.
	// Other potential avenues for custom data:
	//  * UV2
	//  * COLOR (since this is handled through arg_color)
	//  * INSTANCE_CUSTOM (Can only set this through ArrayMesh)
	int uv2x = int(UV2.x);
	if (uv2x == 2) {
		MODELVIEW_MATRIX = INV_CAMERA_MATRIX * mat4(
			CAMERA_MATRIX[0],
			WORLD_MATRIX[1],
			vec4(normalize(cross(CAMERA_MATRIX[0].xyz,WORLD_MATRIX[1].xyz)), 0.0),
			WORLD_MATRIX[3]
		);
		MODELVIEW_MATRIX = MODELVIEW_MATRIX * mat4(
			vec4(1.0, 0.0, 0.0, 0.0),
			vec4(0.0, 1.0/length(WORLD_MATRIX[1].xyz), 0.0, 0.0),
			vec4(0.0, 0.0, 1.0, 0.0),
			vec4(0.0, 0.0, 0.0 ,1.0)
		);
		VERTEX = (MODELVIEW_MATRIX * vec4(VERTEX, 1.0)).xyz;
		NORMAL = (MODELVIEW_MATRIX * vec4(NORMAL, 0.0)).xyz;
	} else if (uv2x == 3) {
		MODELVIEW_MATRIX = INV_CAMERA_MATRIX * mat4(
			CAMERA_MATRIX[0],
			CAMERA_MATRIX[1],
			CAMERA_MATRIX[2],
			WORLD_MATRIX[3]
		);
		VERTEX = (MODELVIEW_MATRIX * vec4(VERTEX, 1.0)).xyz;
		NORMAL = (MODELVIEW_MATRIX * vec4(NORMAL, 0.0)).xyz;
	} else {
		VERTEX = (MODELVIEW_MATRIX * vec4(VERTEX, 1.0)).xyz;
		NORMAL = (MODELVIEW_MATRIX * vec4(NORMAL, 0.0)).xyz;
	}
}

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
		col2 = texture(arg_reflection, reflect(VIEW, normalize(NORMAL)).xy);
	} else {
		col2 = vec4(0, 0, 0, 0);
	}
	vec4 outcol = col1 * vec4(arg_color, arg_alpha);
	outcol.rgb += col2.rgb * col2.a;
	ALBEDO = outcol.rgb;
	ALPHA = outcol.a;
}