// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from dobot_msgs_v4:srv/SetTool485.idl
// generated code does not contain a copyright notice
#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
#include <Python.h>
#include <stdbool.h>
#ifndef _WIN32
# pragma GCC diagnostic push
# pragma GCC diagnostic ignored "-Wunused-function"
#endif
#include "numpy/ndarrayobject.h"
#ifndef _WIN32
# pragma GCC diagnostic pop
#endif
#include "rosidl_runtime_c/visibility_control.h"
#include "dobot_msgs_v4/srv/detail/set_tool485__struct.h"
#include "dobot_msgs_v4/srv/detail/set_tool485__functions.h"

#include "rosidl_runtime_c/string.h"
#include "rosidl_runtime_c/string_functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool dobot_msgs_v4__srv__set_tool485__request__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[50];
    {
      char * class_name = NULL;
      char * module_name = NULL;
      {
        PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
        if (class_attr) {
          PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
          if (name_attr) {
            class_name = (char *)PyUnicode_1BYTE_DATA(name_attr);
            Py_DECREF(name_attr);
          }
          PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
          if (module_attr) {
            module_name = (char *)PyUnicode_1BYTE_DATA(module_attr);
            Py_DECREF(module_attr);
          }
          Py_DECREF(class_attr);
        }
      }
      if (!class_name || !module_name) {
        return false;
      }
      snprintf(full_classname_dest, sizeof(full_classname_dest), "%s.%s", module_name, class_name);
    }
    assert(strncmp("dobot_msgs_v4.srv._set_tool485.SetTool485_Request", full_classname_dest, 49) == 0);
  }
  dobot_msgs_v4__srv__SetTool485_Request * ros_message = _ros_message;
  {  // baudrate
    PyObject * field = PyObject_GetAttrString(_pymsg, "baudrate");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->baudrate = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // parity
    PyObject * field = PyObject_GetAttrString(_pymsg, "parity");
    if (!field) {
      return false;
    }
    assert(PyUnicode_Check(field));
    PyObject * encoded_field = PyUnicode_AsUTF8String(field);
    if (!encoded_field) {
      Py_DECREF(field);
      return false;
    }
    rosidl_runtime_c__String__assign(&ros_message->parity, PyBytes_AS_STRING(encoded_field));
    Py_DECREF(encoded_field);
    Py_DECREF(field);
  }
  {  // stop
    PyObject * field = PyObject_GetAttrString(_pymsg, "stop");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->stop = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // identify
    PyObject * field = PyObject_GetAttrString(_pymsg, "identify");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->identify = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * dobot_msgs_v4__srv__set_tool485__request__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of SetTool485_Request */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("dobot_msgs_v4.srv._set_tool485");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "SetTool485_Request");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  dobot_msgs_v4__srv__SetTool485_Request * ros_message = (dobot_msgs_v4__srv__SetTool485_Request *)raw_ros_message;
  {  // baudrate
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->baudrate);
    {
      int rc = PyObject_SetAttrString(_pymessage, "baudrate", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // parity
    PyObject * field = NULL;
    field = PyUnicode_DecodeUTF8(
      ros_message->parity.data,
      strlen(ros_message->parity.data),
      "replace");
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "parity", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // stop
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->stop);
    {
      int rc = PyObject_SetAttrString(_pymessage, "stop", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // identify
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->identify);
    {
      int rc = PyObject_SetAttrString(_pymessage, "identify", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}

#define NPY_NO_DEPRECATED_API NPY_1_7_API_VERSION
// already included above
// #include <Python.h>
// already included above
// #include <stdbool.h>
// already included above
// #include "numpy/ndarrayobject.h"
// already included above
// #include "rosidl_runtime_c/visibility_control.h"
// already included above
// #include "dobot_msgs_v4/srv/detail/set_tool485__struct.h"
// already included above
// #include "dobot_msgs_v4/srv/detail/set_tool485__functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool dobot_msgs_v4__srv__set_tool485__response__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[51];
    {
      char * class_name = NULL;
      char * module_name = NULL;
      {
        PyObject * class_attr = PyObject_GetAttrString(_pymsg, "__class__");
        if (class_attr) {
          PyObject * name_attr = PyObject_GetAttrString(class_attr, "__name__");
          if (name_attr) {
            class_name = (char *)PyUnicode_1BYTE_DATA(name_attr);
            Py_DECREF(name_attr);
          }
          PyObject * module_attr = PyObject_GetAttrString(class_attr, "__module__");
          if (module_attr) {
            module_name = (char *)PyUnicode_1BYTE_DATA(module_attr);
            Py_DECREF(module_attr);
          }
          Py_DECREF(class_attr);
        }
      }
      if (!class_name || !module_name) {
        return false;
      }
      snprintf(full_classname_dest, sizeof(full_classname_dest), "%s.%s", module_name, class_name);
    }
    assert(strncmp("dobot_msgs_v4.srv._set_tool485.SetTool485_Response", full_classname_dest, 50) == 0);
  }
  dobot_msgs_v4__srv__SetTool485_Response * ros_message = _ros_message;
  {  // res
    PyObject * field = PyObject_GetAttrString(_pymsg, "res");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->res = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * dobot_msgs_v4__srv__set_tool485__response__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of SetTool485_Response */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("dobot_msgs_v4.srv._set_tool485");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "SetTool485_Response");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  dobot_msgs_v4__srv__SetTool485_Response * ros_message = (dobot_msgs_v4__srv__SetTool485_Response *)raw_ros_message;
  {  // res
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->res);
    {
      int rc = PyObject_SetAttrString(_pymessage, "res", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}
