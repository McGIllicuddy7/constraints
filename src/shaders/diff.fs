#version 330

// Input vertex attributes (from vertex shader)
in vec2 fragTexCoord;
in vec4 fragColor;

// Input uniform values
uniform sampler2D texture0;
uniform vec4 colDiffuse;
// Output fragment color
out vec4 finalColor;
uniform int height;
uniform int width;
uniform int kernel_size0;
uniform int divisor0;
uniform int kernel_size1;
uniform int divisor1;
uniform bool b_and_w;
float weight(int x, int y, int divisor){
    float d = x*x+y*y;
    return exp(-d/float(divisor));
}
vec3 gaussian(int kernel_size, int divisor){
    vec3 texelColor = vec3(0.0);
    float total = 1.0;
    for(int y = -kernel_size; y<kernel_size; y++){
        for(int x =-kernel_size; x<kernel_size;x++){
            vec2 coord = fragTexCoord+vec2(float(x)/float(width),float(y/float(height)));
            float w = weight(x,y, divisor);
            total += w;
            texelColor += texture(texture0, coord).rgb*w;
        }
    }
    texelColor/= total;
    float length = sqrt(texelColor.r*texelColor.r+texelColor.g*texelColor.g+texelColor.b*texelColor.b);
    if(b_and_w){
        if(length>0.5){
            texelColor = vec3(sqrt(length));
        } else{
               texelColor = vec3(length*length);
        }

    } if(length>0.5){
        texelColor /=length;
    } else{
        texelColor = vec3(0.0);
    }
    return texelColor;
}

void main()
{
    // Texel color fetching from texture sampler
    vec3 base = gaussian(kernel_size0, divisor0);
    vec3 old = gaussian(kernel_size1, divisor1);
    vec3 texelColor = abs(base-old);
    finalColor = vec4(texelColor, 1.0);
}