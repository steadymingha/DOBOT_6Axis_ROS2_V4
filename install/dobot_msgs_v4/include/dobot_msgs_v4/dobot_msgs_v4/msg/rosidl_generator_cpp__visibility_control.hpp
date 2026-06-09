// generated from rosidl_generator_cpp/resource/rosidl_generator_cpp__visibility_control.hpp.in
// generated code does not contain a copyright notice

#ifndef DOBOT_MSGS_V4__MSG__ROSIDL_GENERATOR_CPP__VISIBILITY_CONTROL_HPP_
#define DOBOT_MSGS_V4__MSG__ROSIDL_GENERATOR_CPP__VISIBILITY_CONTROL_HPP_

#ifdef __cplusplus
extern "C"
{
#endif

// This logic was borrowed (then namespaced) from the examples on the gcc wiki:
//     https://gcc.gnu.org/wiki/Visibility

#if defined _WIN32 || defined __CYGWIN__
  #ifdef __GNUC__
    #define ROSIDL_GENERATOR_CPP_EXPORT_dobot_msgs_v4 __attribute__ ((dllexport))
    #define ROSIDL_GENERATOR_CPP_IMPORT_dobot_msgs_v4 __attribute__ ((dllimport))
  #else
    #define ROSIDL_GENERATOR_CPP_EXPORT_dobot_msgs_v4 __declspec(dllexport)
    #define ROSIDL_GENERATOR_CPP_IMPORT_dobot_msgs_v4 __declspec(dllimport)
  #endif
  #ifdef ROSIDL_GENERATOR_CPP_BUILDING_DLL_dobot_msgs_v4
    #define ROSIDL_GENERATOR_CPP_PUBLIC_dobot_msgs_v4 ROSIDL_GENERATOR_CPP_EXPORT_dobot_msgs_v4
  #else
    #define ROSIDL_GENERATOR_CPP_PUBLIC_dobot_msgs_v4 ROSIDL_GENERATOR_CPP_IMPORT_dobot_msgs_v4
  #endif
#else
  #define ROSIDL_GENERATOR_CPP_EXPORT_dobot_msgs_v4 __attribute__ ((visibility("default")))
  #define ROSIDL_GENERATOR_CPP_IMPORT_dobot_msgs_v4
  #if __GNUC__ >= 4
    #define ROSIDL_GENERATOR_CPP_PUBLIC_dobot_msgs_v4 __attribute__ ((visibility("default")))
  #else
    #define ROSIDL_GENERATOR_CPP_PUBLIC_dobot_msgs_v4
  #endif
#endif

#ifdef __cplusplus
}
#endif

#endif  // DOBOT_MSGS_V4__MSG__ROSIDL_GENERATOR_CPP__VISIBILITY_CONTROL_HPP_
