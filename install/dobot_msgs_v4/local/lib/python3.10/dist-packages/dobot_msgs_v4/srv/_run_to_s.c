// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from dobot_msgs_v4:srv/RunTo.idl
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
#include "dobot_msgs_v4/srv/detail/run_to__struct.h"
#include "dobot_msgs_v4/srv/detail/run_to__functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool dobot_msgs_v4__srv__run_to__request__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[40];
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
    assert(strncmp("dobot_msgs_v4.srv._run_to.RunTo_Request", full_classname_dest, 39) == 0);
  }
  dobot_msgs_v4__srv__RunTo_Request * ros_message = _ros_message;
  {  // a1
    PyObject * field = PyObject_GetAttrString(_pymsg, "a1");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->a1 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // b1
    PyObject * field = PyObject_GetAttrString(_pymsg, "b1");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->b1 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // c1
    PyObject * field = PyObject_GetAttrString(_pymsg, "c1");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->c1 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // d1
    PyObject * field = PyObject_GetAttrString(_pymsg, "d1");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->d1 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // e1
    PyObject * field = PyObject_GetAttrString(_pymsg, "e1");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->e1 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // f1
    PyObject * field = PyObject_GetAttrString(_pymsg, "f1");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->f1 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // move_type
    PyObject * field = PyObject_GetAttrString(_pymsg, "move_type");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->move_type = (int32_t)PyLong_AsLong(field);
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
  {  // a
    PyObject * field = PyObject_GetAttrString(_pymsg, "a");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->a = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }
  {  // v
    PyObject * field = PyObject_GetAttrString(_pymsg, "v");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->v = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * dobot_msgs_v4__srv__run_to__request__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of RunTo_Request */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("dobot_msgs_v4.srv._run_to");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "RunTo_Request");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  dobot_msgs_v4__srv__RunTo_Request * ros_message = (dobot_msgs_v4__srv__RunTo_Request *)raw_ros_message;
  {  // a1
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->a1);
    {
      int rc = PyObject_SetAttrString(_pymessage, "a1", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // b1
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->b1);
    {
      int rc = PyObject_SetAttrString(_pymessage, "b1", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // c1
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->c1);
    {
      int rc = PyObject_SetAttrString(_pymessage, "c1", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // d1
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->d1);
    {
      int rc = PyObject_SetAttrString(_pymessage, "d1", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // e1
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->e1);
    {
      int rc = PyObject_SetAttrString(_pymessage, "e1", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // f1
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->f1);
    {
      int rc = PyObject_SetAttrString(_pymessage, "f1", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // move_type
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->move_type);
    {
      int rc = PyObject_SetAttrString(_pymessage, "move_type", field);
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
  {  // a
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->a);
    {
      int rc = PyObject_SetAttrString(_pymessage, "a", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // v
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->v);
    {
      int rc = PyObject_SetAttrString(_pymessage, "v", field);
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
// #include "dobot_msgs_v4/srv/detail/run_to__struct.h"
// already included above
// #include "dobot_msgs_v4/srv/detail/run_to__functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool dobot_msgs_v4__srv__run_to__response__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[41];
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
    assert(strncmp("dobot_msgs_v4.srv._run_to.RunTo_Response", full_classname_dest, 40) == 0);
  }
  dobot_msgs_v4__srv__RunTo_Response * ros_message = _ros_message;
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
PyObject * dobot_msgs_v4__srv__run_to__response__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of RunTo_Response */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("dobot_msgs_v4.srv._run_to");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "RunTo_Response");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  dobot_msgs_v4__srv__RunTo_Response * ros_message = (dobot_msgs_v4__srv__RunTo_Response *)raw_ros_message;
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
