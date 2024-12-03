#version 330

// Input vertex attributes (from vertex shader)
in vec2 fragTexCoord;
in vec4 fragColor;

// Input uniform values
uniform sampler2D texture0;
uniform vec4 colDiffuse;
// Output fragment color
out vec4 finalColor;
uniform int divisor;
uniform int height;
uniform int width;
uniform int kernel_size;
float weight(int x, int y){
    float d = x*x+y*y;
    return exp(-d/float(divisor));
}
void main()
{
    // Texel color fetching from texture sampler
    vec3 texelColor = vec3(0.0);
    float total = 1.0;
    for(int y = -kernel_size; y<kernel_size; y++){
        for(int x =-kernel_size; x<kernel_size;x++){
            vec2 coord = fragTexCoord+vec2(float(x)/float(width),float(y/float(height)));
            float w = weight(x,y);
            total += w;
            texelColor += texture(texture0, coord).rgb*w;
        }
    }
    texelColor/= total;
    finalColor = vec4(texelColor, 1.0);
}