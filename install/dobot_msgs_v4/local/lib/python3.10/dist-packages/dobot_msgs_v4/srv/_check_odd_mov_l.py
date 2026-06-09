# generated from rosidl_generator_py/resource/_idl.py.em
# with input from dobot_msgs_v4:srv/CheckOddMovL.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import math  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_CheckOddMovL_Request(type):
    """Metaclass of message 'CheckOddMovL_Request'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('dobot_msgs_v4')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'dobot_msgs_v4.srv.CheckOddMovL_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__check_odd_mov_l__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__check_odd_mov_l__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__check_odd_mov_l__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__check_odd_mov_l__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__check_odd_mov_l__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class CheckOddMovL_Request(metaclass=Metaclass_CheckOddMovL_Request):
    """Message class 'CheckOddMovL_Request'."""

    __slots__ = [
        '_point1_j1',
        '_point1_j2',
        '_point1_j3',
        '_point1_j4',
        '_point1_j5',
        '_point1_j6',
        '_point2_j1',
        '_point2_j2',
        '_point2_j3',
        '_point2_j4',
        '_point2_j5',
        '_point2_j6',
        '_param_value',
    ]

    _fields_and_field_types = {
        'point1_j1': 'double',
        'point1_j2': 'double',
        'point1_j3': 'double',
        'point1_j4': 'double',
        'point1_j5': 'double',
        'point1_j6': 'double',
        'point2_j1': 'double',
        'point2_j2': 'double',
        'point2_j3': 'double',
        'point2_j4': 'double',
        'point2_j5': 'double',
        'point2_j6': 'double',
        'param_value': 'sequence<string>',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.UnboundedSequence(rosidl_parser.definition.UnboundedString()),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.point1_j1 = kwargs.get('point1_j1', float())
        self.point1_j2 = kwargs.get('point1_j2', float())
        self.point1_j3 = kwargs.get('point1_j3', float())
        self.point1_j4 = kwargs.get('point1_j4', float())
        self.point1_j5 = kwargs.get('point1_j5', float())
        self.point1_j6 = kwargs.get('point1_j6', float())
        self.point2_j1 = kwargs.get('point2_j1', float())
        self.point2_j2 = kwargs.get('point2_j2', float())
        self.point2_j3 = kwargs.get('point2_j3', float())
        self.point2_j4 = kwargs.get('point2_j4', float())
        self.point2_j5 = kwargs.get('point2_j5', float())
        self.point2_j6 = kwargs.get('point2_j6', float())
        self.param_value = kwargs.get('param_value', [])

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.point1_j1 != other.point1_j1:
            return False
        if self.point1_j2 != other.point1_j2:
            return False
        if self.point1_j3 != other.point1_j3:
            return False
        if self.point1_j4 != other.point1_j4:
            return False
        if self.point1_j5 != other.point1_j5:
            return False
        if self.point1_j6 != other.point1_j6:
            return False
        if self.point2_j1 != other.point2_j1:
            return False
        if self.point2_j2 != other.point2_j2:
            return False
        if self.point2_j3 != other.point2_j3:
            return False
        if self.point2_j4 != other.point2_j4:
            return False
        if self.point2_j5 != other.point2_j5:
            return False
        if self.point2_j6 != other.point2_j6:
            return False
        if self.param_value != other.param_value:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def point1_j1(self):
        """Message field 'point1_j1'."""
        return self._point1_j1

    @point1_j1.setter
    def point1_j1(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'point1_j1' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'point1_j1' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._point1_j1 = value

    @builtins.property
    def point1_j2(self):
        """Message field 'point1_j2'."""
        return self._point1_j2

    @point1_j2.setter
    def point1_j2(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'point1_j2' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'point1_j2' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._point1_j2 = value

    @builtins.property
    def point1_j3(self):
        """Message field 'point1_j3'."""
        return self._point1_j3

    @point1_j3.setter
    def point1_j3(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'point1_j3' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'point1_j3' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._point1_j3 = value

    @builtins.property
    def point1_j4(self):
        """Message field 'point1_j4'."""
        return self._point1_j4

    @point1_j4.setter
    def point1_j4(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'point1_j4' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'point1_j4' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._point1_j4 = value

    @builtins.property
    def point1_j5(self):
        """Message field 'point1_j5'."""
        return self._point1_j5

    @point1_j5.setter
    def point1_j5(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'point1_j5' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'point1_j5' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._point1_j5 = value

    @builtins.property
    def point1_j6(self):
        """Message field 'point1_j6'."""
        return self._point1_j6

    @point1_j6.setter
    def point1_j6(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'point1_j6' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'point1_j6' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._point1_j6 = value

    @builtins.property
    def point2_j1(self):
        """Message field 'point2_j1'."""
        return self._point2_j1

    @point2_j1.setter
    def point2_j1(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'point2_j1' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'point2_j1' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._point2_j1 = value

    @builtins.property
    def point2_j2(self):
        """Message field 'point2_j2'."""
        return self._point2_j2

    @point2_j2.setter
    def point2_j2(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'point2_j2' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'point2_j2' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._point2_j2 = value

    @builtins.property
    def point2_j3(self):
        """Message field 'point2_j3'."""
        return self._point2_j3

    @point2_j3.setter
    def point2_j3(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'point2_j3' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'point2_j3' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._point2_j3 = value

    @builtins.property
    def point2_j4(self):
        """Message field 'point2_j4'."""
        return self._point2_j4

    @point2_j4.setter
    def point2_j4(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'point2_j4' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'point2_j4' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._point2_j4 = value

    @builtins.property
    def point2_j5(self):
        """Message field 'point2_j5'."""
        return self._point2_j5

    @point2_j5.setter
    def point2_j5(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'point2_j5' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'point2_j5' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._point2_j5 = value

    @builtins.property
    def point2_j6(self):
        """Message field 'point2_j6'."""
        return self._point2_j6

    @point2_j6.setter
    def point2_j6(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'point2_j6' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'point2_j6' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._point2_j6 = value

    @builtins.property
    def param_value(self):
        """Message field 'param_value'."""
        return self._param_value

    @param_value.setter
    def param_value(self, value):
        if __debug__:
            from collections.abc import Sequence
            from collections.abc import Set
            from collections import UserList
            from collections import UserString
            assert \
                ((isinstance(value, Sequence) or
                  isinstance(value, Set) or
                  isinstance(value, UserList)) and
                 not isinstance(value, str) and
                 not isinstance(value, UserString) and
                 all(isinstance(v, str) for v in value) and
                 True), \
                "The 'param_value' field must be a set or sequence and each value of type 'str'"
        self._param_value = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_CheckOddMovL_Response(type):
    """Metaclass of message 'CheckOddMovL_Response'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('dobot_msgs_v4')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'dobot_msgs_v4.srv.CheckOddMovL_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__check_odd_mov_l__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__check_odd_mov_l__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__check_odd_mov_l__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__check_odd_mov_l__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__check_odd_mov_l__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class CheckOddMovL_Response(metaclass=Metaclass_CheckOddMovL_Response):
    """Message class 'CheckOddMovL_Response'."""

    __slots__ = [
        '_robot_return',
        '_res',
    ]

    _fields_and_field_types = {
        'robot_return': 'string',
        'res': 'int32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.robot_return = kwargs.get('robot_return', str())
        self.res = kwargs.get('res', int())

    def __repr__(self):
        typename = self.__class__.__module__.split('.')
        typename.pop()
        typename.append(self.__class__.__name__)
        args = []
        for s, t in zip(self.__slots__, self.SLOT_TYPES):
            field = getattr(self, s)
            fieldstr = repr(field)
            # We use Python array type for fields that can be directly stored
            # in them, and "normal" sequences for everything else.  If it is
            # a type that we store in an array, strip off the 'array' portion.
            if (
                isinstance(t, rosidl_parser.definition.AbstractSequence) and
                isinstance(t.value_type, rosidl_parser.definition.BasicType) and
                t.value_type.typename in ['float', 'double', 'int8', 'uint8', 'int16', 'uint16', 'int32', 'uint32', 'int64', 'uint64']
            ):
                if len(field) == 0:
                    fieldstr = '[]'
                else:
                    assert fieldstr.startswith('array(')
                    prefix = "array('X', "
                    suffix = ')'
                    fieldstr = fieldstr[len(prefix):-len(suffix)]
            args.append(s[1:] + '=' + fieldstr)
        return '%s(%s)' % ('.'.join(typename), ', '.join(args))

    def __eq__(self, other):
        if not isinstance(other, self.__class__):
            return False
        if self.robot_return != other.robot_return:
            return False
        if self.res != other.res:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def robot_return(self):
        """Message field 'robot_return'."""
        return self._robot_return

    @robot_return.setter
    def robot_return(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'robot_return' field must be of type 'str'"
        self._robot_return = value

    @builtins.property
    def res(self):
        """Message field 'res'."""
        return self._res

    @res.setter
    def res(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'res' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'res' field must be an integer in [-2147483648, 2147483647]"
        self._res = value


class Metaclass_CheckOddMovL(type):
    """Metaclass of service 'CheckOddMovL'."""

    _TYPE_SUPPORT = None

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('dobot_msgs_v4')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'dobot_msgs_v4.srv.CheckOddMovL')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__check_odd_mov_l

            from dobot_msgs_v4.srv import _check_odd_mov_l
            if _check_odd_mov_l.Metaclass_CheckOddMovL_Request._TYPE_SUPPORT is None:
                _check_odd_mov_l.Metaclass_CheckOddMovL_Request.__import_type_support__()
            if _check_odd_mov_l.Metaclass_CheckOddMovL_Response._TYPE_SUPPORT is None:
                _check_odd_mov_l.Metaclass_CheckOddMovL_Response.__import_type_support__()


class CheckOddMovL(metaclass=Metaclass_CheckOddMovL):
    from dobot_msgs_v4.srv._check_odd_mov_l import CheckOddMovL_Request as Request
    from dobot_msgs_v4.srv._check_odd_mov_l import CheckOddMovL_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
