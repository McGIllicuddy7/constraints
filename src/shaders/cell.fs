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

float weight(int x, int y, int divisor){
    float d = x*x+y*y;
    return exp(-d/float(divisor));
}
vec3 dither(vec3 color){
    int amnt = 10;
    float phi = acos(color.r);
    float theta = 0.0;
    if(phi != 0.0){
        theta = atan(color.b/color.g);
    }
    float phi_0 = round(phi*amnt)/amnt;
    float theta_0 = round(theta*amnt)/amnt;
    vec3 rtv = vec3(cos(phi_0), sin(phi_0)*cos(theta_0), sin(phi_0)*sin(theta_0));
    return rtv;
}
vec3 gaussian(int kernel_size0, int divisor0){
    vec3 texelColor = vec3(0.0);
    float total = 1.0;
    for(int y = -kernel_size0; y<kernel_size0; y++){
        for(int x =-kernel_size0; x<kernel_size0;x++){
            vec2 coord = fragTexCoord+vec2(float(x)/float(width),float(y/float(height)));
            float w = weight(x,y, divisor0);
            total += w;
            texelColor += texture(texture0, coord).rgb*w;
        }
    }
    texelColor/= total;
    return texelColor;
}
void main()
{
    // Texel color fetching from texture sampler
    vec3 texelColor = gaussian(1, 100);
    float length = sqrt(texelColor.r*texelColor.r+texelColor.g*texelColor.g+texelColor.b*texelColor.b);
    float delta = 0.15;
    if(length<delta/2.0){
        texelColor *= 0.0;
    } else{
        if(length>0.999999){
            texelColor = vec3(1.0);
        } else{
            vec3 normalized = texelColor/length;
            normalized = dither(normalized);
            float dt = delta*2.0;
            while(dt<length){
                dt += delta;
            }

            texelColor  = normalized*(dt-delta);
        }
    }
    finalColor = vec4(texelColor, 1.0);
}