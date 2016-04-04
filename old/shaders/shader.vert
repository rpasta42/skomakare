#version 140
#extension GL_ARB_explicit_attrib_location : require
layout(location = 0) in vec4 position;
//uniform vec3 offset;
void main()
{
   gl_PointSize = 3;
   //gl_Position = position;
   //gl_Position = vec4(position.x, position.y, position.z, 10);
   gl_Position = vec4(position.x, position.z, position.y, 4);
   
//   gl_Position = position + vec4(offset.x, offset.y, offset.z, 0);
}
