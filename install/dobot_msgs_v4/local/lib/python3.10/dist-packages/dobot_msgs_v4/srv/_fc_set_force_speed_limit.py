# generated from rosidl_generator_py/resource/_idl.py.em
# with input from dobot_msgs_v4:srv/FCSetForceSpeedLimit.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_FCSetForceSpeedLimit_Request(type):
    """Metaclass of message 'FCSetForceSpeedLimit_Request'."""

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
                'dobot_msgs_v4.srv.FCSetForceSpeedLimit_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__fc_set_force_speed_limit__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__fc_set_force_speed_limit__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__fc_set_force_speed_limit__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__fc_set_force_speed_limit__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__fc_set_force_speed_limit__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class FCSetForceSpeedLimit_Request(metaclass=Metaclass_FCSetForceSpeedLimit_Request):
    """Message class 'FCSetForceSpeedLimit_Request'."""

    __slots__ = [
        '_x',
        '_y',
        '_z',
        '_rx',
        '_ry',
        '_rz',
    ]

    _fields_and_field_types = {
        'x': 'int32',
        'y': 'int32',
        'z': 'int32',
        'rx': 'int32',
        'ry': 'int32',
        'rz': 'int32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.x = kwargs.get('x', int())
        self.y = kwargs.get('y', int())
        self.z = kwargs.get('z', int())
        self.rx = kwargs.get('rx', int())
        self.ry = kwargs.get('ry', int())
        self.rz = kwargs.get('rz', int())

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
        if self.x != other.x:
            return False
        if self.y != other.y:
            return False
        if self.z != other.z:
            return False
        if self.rx != other.rx:
            return False
        if self.ry != other.ry:
            return False
        if self.rz != other.rz:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def x(self):
        """Message field 'x'."""
        return self._x

    @x.setter
    def x(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'x' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'x' field must be an integer in [-2147483648, 2147483647]"
        self._x = value

    @builtins.property
    def y(self):
        """Message field 'y'."""
        return self._y

    @y.setter
    def y(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'y' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'y' field must be an integer in [-2147483648, 2147483647]"
        self._y = value

    @builtins.property
    def z(self):
        """Message field 'z'."""
        return self._z

    @z.setter
    def z(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'z' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'z' field must be an integer in [-2147483648, 2147483647]"
        self._z = value

    @builtins.property
    def rx(self):
        """Message field 'rx'."""
        return self._rx

    @rx.setter
    def rx(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'rx' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'rx' field must be an integer in [-2147483648, 2147483647]"
        self._rx = value

    @builtins.property
    def ry(self):
        """Message field 'ry'."""
        return self._ry

    @ry.setter
    def ry(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'ry' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'ry' field must be an integer in [-2147483648, 2147483647]"
        self._ry = value

    @builtins.property
    def rz(self):
        """Message field 'rz'."""
        return self._rz

    @rz.setter
    def rz(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'rz' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'rz' field must be an integer in [-2147483648, 2147483647]"
        self._rz = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_FCSetForceSpeedLimit_Response(type):
    """Metaclass of message 'FCSetForceSpeedLimit_Response'."""

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
                'dobot_msgs_v4.srv.FCSetForceSpeedLimit_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__fc_set_force_speed_limit__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__fc_set_force_speed_limit__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__fc_set_force_speed_limit__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__fc_set_force_speed_limit__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__fc_set_force_speed_limit__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class FCSetForceSpeedLimit_Response(metaclass=Metaclass_FCSetForceSpeedLimit_Response):
    """Message class 'FCSetForceSpeedLimit_Response'."""

    __slots__ = [
        '_res',
    ]

    _fields_and_field_types = {
        'res': 'int32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
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
        if self.res != other.res:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

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


class Metaclass_FCSetForceSpeedLimit(type):
    """Metaclass of service 'FCSetForceSpeedLimit'."""

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
                'dobot_msgs_v4.srv.FCSetForceSpeedLimit')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__fc_set_force_speed_limit

            from dobot_msgs_v4.srv import _fc_set_force_speed_limit
            if _fc_set_force_speed_limit.Metaclass_FCSetForceSpeedLimit_Request._TYPE_SUPPORT is None:
                _fc_set_force_speed_limit.Metaclass_FCSetForceSpeedLimit_Request.__import_type_support__()
            if _fc_set_force_speed_limit.Metaclass_FCSetForceSpeedLimit_Response._TYPE_SUPPORT is None:
                _fc_set_force_speed_limit.Metaclass_FCSetForceSpeedLimit_Response.__import_type_support__()


class FCSetForceSpeedLimit(metaclass=Metaclass_FCSetForceSpeedLimit):
    from dobot_msgs_v4.srv._fc_set_force_speed_limit import FCSetForceSpeedLimit_Request as Request
    from dobot_msgs_v4.srv._fc_set_force_speed_limit import FCSetForceSpeedLimit_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
