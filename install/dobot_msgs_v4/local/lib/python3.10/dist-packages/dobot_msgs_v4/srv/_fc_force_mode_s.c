// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from dobot_msgs_v4:srv/FCForceMode.idl
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
#include "dobot_msgs_v4/srv/detail/fc_force_mode__struct.h"
#include "dobot_msgs_v4/srv/detail/fc_force_mode__functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool dobot_msgs_v4__srv__fc_force_mode__request__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[53];
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
    assert(strncmp("dobot_msgs_v4.srv._fc_force_mode.FCForceMode_Request", full_classname_dest, 52) == 0);
  }
  dobot_msgs_v4__srv__FCForceMode_Request * ros_message = _ros_message;
  {  // x
    PyObject * field = PyObject_GetAttrString(_pymsg, "x");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->x = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // y
    PyObject * field = PyObject_GetAttrString(_pymsg, "y");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->y = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // z
    PyObject * field = PyObject_GetAttrString(_pymsg, "z");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->z = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // rx
    PyObject * field = PyObject_GetAttrString(_pymsg, "rx");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->rx = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // ry
    PyObject * field = PyObject_GetAttrString(_pymsg, "ry");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->ry = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // rz
    PyObject * field = PyObject_GetAttrString(_pymsg, "rz");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->rz = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // fx
    PyObject * field = PyObject_GetAttrString(_pymsg, "fx");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->fx = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // fy
    PyObject * field = PyObject_GetAttrString(_pymsg, "fy");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->fy = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // fz
    PyObject * field = PyObject_GetAttrString(_pymsg, "fz");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->fz = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // frx
    PyObject * field = PyObject_GetAttrString(_pymsg, "frx");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->frx = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // fry
    PyObject * field = PyObject_GetAttrString(_pymsg, "fry");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->fry = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // frz
    PyObject * field = PyObject_GetAttrString(_pymsg, "frz");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->frz = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // reference
    PyObject * field = PyObject_GetAttrString(_pymsg, "reference");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->reference = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // user
    PyObject * field = PyObject_GetAttrString(_pymsg, "user");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->user = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // tool
    PyObject * field = PyObject_GetAttrString(_pymsg, "tool");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->tool = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * dobot_msgs_v4__srv__fc_force_mode__request__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of FCForceMode_Request */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("dobot_msgs_v4.srv._fc_force_mode");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "FCForceMode_Request");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  dobot_msgs_v4__srv__FCForceMode_Request * ros_message = (dobot_msgs_v4__srv__FCForceMode_Request *)raw_ros_message;
  {  // x
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->x);
    {
      int rc = PyObject_SetAttrString(_pymessage, "x", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // y
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->y);
    {
      int rc = PyObject_SetAttrString(_pymessage, "y", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // z
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->z);
    {
      int rc = PyObject_SetAttrString(_pymessage, "z", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // rx
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->rx);
    {
      int rc = PyObject_SetAttrString(_pymessage, "rx", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // ry
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->ry);
    {
      int rc = PyObject_SetAttrString(_pymessage, "ry", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // rz
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->rz);
    {
      int rc = PyObject_SetAttrString(_pymessage, "rz", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // fx
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->fx);
    {
      int rc = PyObject_SetAttrString(_pymessage, "fx", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // fy
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->fy);
    {
      int rc = PyObject_SetAttrString(_pymessage, "fy", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // fz
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->fz);
    {
      int rc = PyObject_SetAttrString(_pymessage, "fz", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // frx
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->frx);
    {
      int rc = PyObject_SetAttrString(_pymessage, "frx", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // fry
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->fry);
    {
      int rc = PyObject_SetAttrString(_pymessage, "fry", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // frz
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->frz);
    {
      int rc = PyObject_SetAttrString(_pymessage, "frz", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // reference
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->reference);
    {
      int rc = PyObject_SetAttrString(_pymessage, "reference", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // user
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->user);
    {
      int rc = PyObject_SetAttrString(_pymessage, "user", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // tool
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->tool);
    {
      int rc = PyObject_SetAttrString(_pymessage, "tool", field);
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
// #include "dobot_msgs_v4/srv/detail/fc_force_mode__struct.h"
// already included above
// #include "dobot_msgs_v4/srv/detail/fc_force_mode__functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool dobot_msgs_v4__srv__fc_force_mode__response__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[54];
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
    assert(strncmp("dobot_msgs_v4.srv._fc_force_mode.FCForceMode_Response", full_classname_dest, 53) == 0);
  }
  dobot_msgs_v4__srv__FCForceMode_Response * ros_message = _ros_message;
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
PyObject * dobot_msgs_v4__srv__fc_force_mode__response__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of FCForceMode_Response */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("dobot_msgs_v4.srv._fc_force_mode");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "FCForceMode_Response");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  dobot_msgs_v4__srv__FCForceMode_Response * ros_message = (dobot_msgs_v4__srv__FCForceMode_Response *)raw_ros_message;
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
