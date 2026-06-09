// generated from rosidl_generator_py/resource/_idl_support.c.em
// with input from ariac_interfaces:msg/CompetitionStatus.idl
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
#include "ariac_interfaces/msg/detail/competition_status__struct.h"
#include "ariac_interfaces/msg/detail/competition_status__functions.h"

bool ariac_interfaces__msg__competition_time__convert_from_py(PyObject * _pymsg, void * _ros_message);
PyObject * ariac_interfaces__msg__competition_time__convert_to_py(void * raw_ros_message);

ROSIDL_GENERATOR_C_EXPORT
bool ariac_interfaces__msg__competition_status__convert_from_py(PyObject * _pymsg, void * _ros_message)
{
  // check that the passed message is of the expected Python class
  {
    char full_classname_dest[59];
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
    assert(strncmp("ariac_interfaces.msg._competition_status.CompetitionStatus", full_classname_dest, 58) == 0);
  }
  ariac_interfaces__msg__CompetitionStatus * ros_message = _ros_message;
  {  // competition_state
    PyObject * field = PyObject_GetAttrString(_pymsg, "competition_state");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->competition_state = (uint8_t)PyLong_AsUnsignedLong(field);
    Py_DECREF(field);
  }
  {  // num_kits
    PyObject * field = PyObject_GetAttrString(_pymsg, "num_kits");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->num_kits = (uint8_t)PyLong_AsUnsignedLong(field);
    Py_DECREF(field);
  }
  {  // num_modules
    PyObject * field = PyObject_GetAttrString(_pymsg, "num_modules");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->num_modules = (uint8_t)PyLong_AsUnsignedLong(field);
    Py_DECREF(field);
  }
  {  // num_kits_remaining
    PyObject * field = PyObject_GetAttrString(_pymsg, "num_kits_remaining");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->num_kits_remaining = (uint8_t)PyLong_AsUnsignedLong(field);
    Py_DECREF(field);
  }
  {  // num_modules_remaining
    PyObject * field = PyObject_GetAttrString(_pymsg, "num_modules_remaining");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->num_modules_remaining = (uint8_t)PyLong_AsUnsignedLong(field);
    Py_DECREF(field);
  }
  {  // time
    PyObject * field = PyObject_GetAttrString(_pymsg, "time");
    if (!field) {
      return false;
    }
    if (!ariac_interfaces__msg__competition_time__convert_from_py(field, &ros_message->time)) {
      Py_DECREF(field);
      return false;
    }
    Py_DECREF(field);
  }
  {  // run_id
    PyObject * field = PyObject_GetAttrString(_pymsg, "run_id");
    if (!field) {
      return false;
    }
    assert(PyLong_Check(field));
    ros_message->run_id = (int32_t)PyLong_AsLong(field);
    Py_DECREF(field);
  }

  return true;
}

ROSIDL_GENERATOR_C_EXPORT
PyObject * ariac_interfaces__msg__competition_status__convert_to_py(void * raw_ros_message)
{
  /* NOTE(esteve): Call constructor of CompetitionStatus */
  PyObject * _pymessage = NULL;
  {
    PyObject * pymessage_module = PyImport_ImportModule("ariac_interfaces.msg._competition_status");
    assert(pymessage_module);
    PyObject * pymessage_class = PyObject_GetAttrString(pymessage_module, "CompetitionStatus");
    assert(pymessage_class);
    Py_DECREF(pymessage_module);
    _pymessage = PyObject_CallObject(pymessage_class, NULL);
    Py_DECREF(pymessage_class);
    if (!_pymessage) {
      return NULL;
    }
  }
  ariac_interfaces__msg__CompetitionStatus * ros_message = (ariac_interfaces__msg__CompetitionStatus *)raw_ros_message;
  {  // competition_state
    PyObject * field = NULL;
    field = PyLong_FromUnsignedLong(ros_message->competition_state);
    {
      int rc = PyObject_SetAttrString(_pymessage, "competition_state", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // num_kits
    PyObject * field = NULL;
    field = PyLong_FromUnsignedLong(ros_message->num_kits);
    {
      int rc = PyObject_SetAttrString(_pymessage, "num_kits", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // num_modules
    PyObject * field = NULL;
    field = PyLong_FromUnsignedLong(ros_message->num_modules);
    {
      int rc = PyObject_SetAttrString(_pymessage, "num_modules", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // num_kits_remaining
    PyObject * field = NULL;
    field = PyLong_FromUnsignedLong(ros_message->num_kits_remaining);
    {
      int rc = PyObject_SetAttrString(_pymessage, "num_kits_remaining", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // num_modules_remaining
    PyObject * field = NULL;
    field = PyLong_FromUnsignedLong(ros_message->num_modules_remaining);
    {
      int rc = PyObject_SetAttrString(_pymessage, "num_modules_remaining", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // time
    PyObject * field = NULL;
    field = ariac_interfaces__msg__competition_time__convert_to_py(&ros_message->time);
    if (!field) {
      return NULL;
    }
    {
      int rc = PyObject_SetAttrString(_pymessage, "time", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }
  {  // run_id
    PyObject * field = NULL;
    field = PyLong_FromLong(ros_message->run_id);
    {
      int rc = PyObject_SetAttrString(_pymessage, "run_id", field);
      Py_DECREF(field);
      if (rc) {
        return NULL;
      }
    }
  }

  // ownership of _pymessage is transferred to the caller
  return _pymessage;
}
