#version 330

// Input vertex attributes (from vertex shader)
in vec2 fragTexCoord;
in vec4 fragColor;

// Input uniform values
uniform sampler2D texture0;
uniform vec4 colDiffuse;
uniform int kernel_size;
uniform float divisor;
uniform float texture_height;
uniform float texture_width;
// Output fragment color
out vec4 finalColor;

float weight(int x, int y){
    float d = x*x+y*y;
    return exp(-d/10000);
}
void main()
{
    // Texel color fetching from texture sampler
    vec3 texelColor = vec3(0.0);
    float total = 0.0;
    for(int y = -kernel_size;y<kernel_size;y++){
        for(int x = kernal_size; x<kernel_size; x++){
            vec2 coord = fragTexCoord+vec2(float(x),float(y));
            float w = weight(x,y);
            total += w;
            texelColor += texture(texture0, coord).rgb*w;
        }
    }
    texelColor /= total;
/*
    for (int i = 1; i < 3; i++)
    {
        texelColor += texture(texture0, fragTexCoord + vec2(offset[i])/renderWidth, 0.0).rgb*weight[i];
        texelColor += texture(texture0, fragTexCoord - vec2(offset[i])/renderWidth, 0.0).rgb*weight[i];
    }
*/
    finalColor = vec4(texelColor, 1.0);
}