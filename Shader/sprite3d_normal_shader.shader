shader_type spatial;
render_mode depth_draw_opaque,unshaded,skip_vertex_transform;
uniform vec4 albedo : hint_color;
uniform sampler2D texture_albedo : hint_albedo;
uniform float alpha_scissor_threshold;
uniform vec4 param_color: hint_color = vec4(1.0);

void vertex() {
	MODELVIEW_MATRIX = INV_CAMERA_MATRIX * mat4(CAMERA_MATRIX[0],CAMERA_MATRIX[1],CAMERA_MATRIX[2],WORLD_MATRIX[3]);
	VERTEX = (MODELVIEW_MATRIX * vec4(VERTEX, 1.0)).xyz;
	NORMAL = (MODELVIEW_MATRIX * vec4(NORMAL, 0.0)).xyz;
	vec2 amt = (UV - vec2(0.5, 0.5)) * 0.25;
	VERTEX.xy += vec2(amt.x, -amt.y);
}

void fragment() {
	vec2 base_uv = UV;
	vec2 uvb2 = (UV - vec2(0.5, 0.5)) * 1.1 + vec2(0.5, 0.5);
	vec4 albedo_tex_base = texture(texture_albedo, base_uv);
	albedo_tex_base = param_color * albedo_tex_base.a;
	vec4 albedo_tex = texture(texture_albedo, uvb2);
	if (uvb2.x < 0.0 || uvb2.x > 1.0 || uvb2.y < 0.0 || uvb2.y > 1.0) {
		albedo_tex.a = 0.0;
	}
	albedo_tex = mix(albedo_tex_base, albedo_tex, albedo_tex.a);
	ALBEDO = albedo.rgb * albedo_tex.rgb;
	ALPHA = albedo.a * albedo_tex.a;
	ALPHA_SCISSOR=alpha_scissor_threshold;
}
