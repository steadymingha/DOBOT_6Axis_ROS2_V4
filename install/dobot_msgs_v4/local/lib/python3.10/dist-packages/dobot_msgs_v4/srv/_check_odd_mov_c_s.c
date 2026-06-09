// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from dobot_msgs_v4:srv/CheckOddMovC.idl
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
#include "dobot_msgs_v4/srv/detail/check_odd_mov_c__struct.h"
#include "dobot_msgs_v4/srv/detail/check_odd_mov_c__functions.h"

#include "rosidl_runtime_c/primitives_sequence.h"
#include "rosidl_runtime_c/primitives_sequence_functions.h"
#include "rosidl_runtime_c/string.h"
#include "rosidl_runtime_c/string_functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool dobot_msgs_v4__srv__check_odd_mov_c__request__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[56];
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
    assert(strncmp("dobot_msgs_v4.srv._check_odd_mov_c.CheckOddMovC_Request", full_classname_dest, 55) == 0);
  }
  dobot_msgs_v4__srv__CheckOddMovC_Request * ros_message = _ros_message;
  {  // point1_j1
    PyObject * field = PyObject_GetAttrString(_pymsg, "point1_j1");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point1_j1 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point1_j2
    PyObject * field = PyObject_GetAttrString(_pymsg, "point1_j2");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point1_j2 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point1_j3
    PyObject * field = PyObject_GetAttrString(_pymsg, "point1_j3");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point1_j3 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point1_j4
    PyObject * field = PyObject_GetAttrString(_pymsg, "point1_j4");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point1_j4 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point1_j5
    PyObject * field = PyObject_GetAttrString(_pymsg, "point1_j5");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point1_j5 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point1_j6
    PyObject * field = PyObject_GetAttrString(_pymsg, "point1_j6");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point1_j6 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point2_j1
    PyObject * field = PyObject_GetAttrString(_pymsg, "point2_j1");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point2_j1 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point2_j2
    PyObject * field = PyObject_GetAttrString(_pymsg, "point2_j2");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point2_j2 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point2_j3
    PyObject * field = PyObject_GetAttrString(_pymsg, "point2_j3");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point2_j3 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point2_j4
    PyObject * field = PyObject_GetAttrString(_pymsg, "point2_j4");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point2_j4 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point2_j5
    PyObject * field = PyObject_GetAttrString(_pymsg, "point2_j5");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point2_j5 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point2_j6
    PyObject * field = PyObject_GetAttrString(_pymsg, "point2_j6");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point2_j6 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point3_j1
    PyObject * field = PyObject_GetAttrString(_pymsg, "point3_j1");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point3_j1 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point3_j2
    PyObject * field = PyObject_GetAttrString(_pymsg, "point3_j2");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point3_j2 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point3_j3
    PyObject * field = PyObject_GetAttrString(_pymsg, "point3_j3");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point3_j3 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point3_j4
    PyObject * field = PyObject_GetAttrString(_pymsg, "point3_j4");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point3_j4 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point3_j5
    PyObject * field = PyObject_GetAttrString(_pymsg, "point3_j5");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point3_j5 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // point3_j6
    PyObject * field = PyObject_GetAttrString(_pymsg, "point3_j6");
    if (!field) {
      return false;
    }
    assert(PyFloat_Check(field));
    ros_message->point3_j6 = PyFloat_AS_DOUBLE(field);
    Py_DECREF(field);
  }
  {  // param_value
    PyObject * field = PyObject_GetAttrString(_pymsg, "param_value");
    if (!field) {
      return false;
    }
    {
      PyObject * seq_field = PySequence_Fast(field, "expected a sequence in 'param_value'");
      if (!seq_field) {
        Py_DECREF(field);
        return false;
      }
      Py_ssize_t size = PySequence_Size(field);
      if (-1 == size) {
        Py_DECREF(seq_field);
        Py_DECREF(field);
        return false;
      }
      if (!rosidl_runtime_c__String__Sequence__init(&(ros_message->param_value), size)) {
        PyErr_SetString(PyExc_RuntimeError, "unable to create String__Sequence ros_message");
        Py_DECREF(seq_field);
        Py_DECREF(field);
        return false;
      }
      rosidl_runtime_c__String * dest = ros_message->param_value.data;
      for (Py_ssize_t i = 0; i < size; ++i) {
        PyObject * item = PySequence_Fast_GET_ITEM(seq_field, i);
        if (!item) {
          Py_DECREF(seq_field);
          Py_DECREF(field);
          return false;
        }
        assert(PyUnicode_Check(item));
        PyObject * encoded_item = PyUnicode_AsUTF8String(item);
        if (!encoded_item) {
          Py_DECREF(seq_field);
          Py_DECREF(field);
          return false;
        }
        rosidl_runtime_c__String__assign(&dest[i], PyBytes_AS_STRING(encoded_item));
        Py_DECREF(encoded_item);
      }
      Py_DECREF(seq_field);
    }
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * dobot_msgs_v4__srv__check_odd_mov_c__request__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of CheckOddMovC_Request */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("dobot_msgs_v4.srv._check_odd_mov_c");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "CheckOddMovC_Request");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  dobot_msgs_v4__srv__CheckOddMovC_Request * ros_message = (dobot_msgs_v4__srv__CheckOddMovC_Request *)raw_ros_message;
  {  // point1_j1
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point1_j1);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point1_j1", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point1_j2
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point1_j2);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point1_j2", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point1_j3
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point1_j3);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point1_j3", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point1_j4
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point1_j4);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point1_j4", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point1_j5
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point1_j5);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point1_j5", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point1_j6
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point1_j6);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point1_j6", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point2_j1
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point2_j1);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point2_j1", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point2_j2
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point2_j2);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point2_j2", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point2_j3
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point2_j3);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point2_j3", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point2_j4
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point2_j4);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point2_j4", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point2_j5
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point2_j5);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point2_j5", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point2_j6
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point2_j6);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point2_j6", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point3_j1
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point3_j1);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point3_j1", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point3_j2
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point3_j2);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point3_j2", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point3_j3
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point3_j3);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point3_j3", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point3_j4
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point3_j4);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point3_j4", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point3_j5
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point3_j5);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point3_j5", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // point3_j6
    PyObject * field = NULL;
    field = PyFloat_FromDouble(ros_message->point3_j6);
    {
      int rc = PyObject_SetAttrString(_pymessage, "point3_j6", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // param_value
    PyObject * field = NULL;
    size_t size = ros_message->param_value.size;
    rosidl_runtime_c__String * src = ros_message->param_value.data;
    field = PyList_New(size);
    if (!field) {
      return NULL;
    }
    for (size_t i = 0; i < size; ++i) {
      PyObject * decoded_item = PyUnicode_DecodeUTF8(src[i].data, strlen(src[i].data), "replace");
      if (!decoded_item) {
        return NULL;
      }
      int rc = PyList_SetItem(field, i, decoded_item);
      (void)rc;
      assert(rc == 0);
    }
    assert(PySequence_Check(field));
    {
      int rc = PyObject_SetAttrString(_pymessage, "param_value", field);
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
// #include "dobot_msgs_v4/srv/detail/check_odd_mov_c__struct.h"
// already included above
// #include "dobot_msgs_v4/srv/detail/check_odd_mov_c__functions.h"

// already included above
// #include "rosidl_runtime_c/string.h"
// already included above
// #include "rosidl_runtime_c/string_functions.h"


ROSIDL_GENERATOR_C_EXPORT
bool dobot_msgs_v4__srv__check_odd_mov_c__response__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[57];
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
    assert(strncmp("dobot_msgs_v4.srv._check_odd_mov_c.CheckOddMovC_Response", full_classname_dest, 56) == 0);
  }
  dobot_msgs_v4__srv__CheckOddMovC_Response * ros_message = _ros_message;
  {  // robot_return
    PyObject * field = PyObject_GetAttrString(_pymsg, "robot_return");
    if (!field) {
      return false;
    }
    assert(PyUnicode_Check(field));
    PyObject * encoded_field = PyUnicode_AsUTF8String(field);
    if (!encoded_field) {
      Py_DECREF(field);
      return false;
    }
    rosidl_runtime_c__String__assign(&ros_message->robot_return, PyBytes_AS_STRING(encoded_field));
    Py_DECREF(encoded_field);
    Py_DECREF(field);
  }
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
PyObject * dobot_msgs_v4__srv__check_odd_mov_c__response__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of CheckOddMovC_Response */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("dobot_msgs_v4.srv._check_odd_mov_c");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "CheckOddMovC_Response");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  dobot_msgs_v4__srv__CheckOddMovC_Response * ros_message = (dobot_msgs_v4__srv__CheckOddMovC_Response *)raw_ros_message;
  {  // robot_return
    PyObject * field = NULL;
    field = PyUnicode_DecodeUTF8(
      ros_message->robot_return.data,
      strlen(ros_message->robot_return.data),
      "replace");
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "robot_return", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
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
