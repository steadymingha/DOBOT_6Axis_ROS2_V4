# generated from rosidl_generator_py/resource/_idl.py.em
# with input from dobot_msgs_v4:srv/FCForceMode.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_FCForceMode_Request(type):
    """Metaclass of message 'FCForceMode_Request'."""

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
                'dobot_msgs_v4.srv.FCForceMode_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__fc_force_mode__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__fc_force_mode__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__fc_force_mode__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__fc_force_mode__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__fc_force_mode__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'REFERENCE__DEFAULT': -1,
            'USER__DEFAULT': -1,
            'TOOL__DEFAULT': -1,
        }

    @property
    def REFERENCE__DEFAULT(cls):
        """Return default value for message field 'reference'."""
        return -1

    @property
    def USER__DEFAULT(cls):
        """Return default value for message field 'user'."""
        return -1

    @property
    def TOOL__DEFAULT(cls):
        """Return default value for message field 'tool'."""
        return -1


class FCForceMode_Request(metaclass=Metaclass_FCForceMode_Request):
    """Message class 'FCForceMode_Request'."""

    __slots__ = [
        '_x',
        '_y',
        '_z',
        '_rx',
        '_ry',
        '_rz',
        '_fx',
        '_fy',
        '_fz',
        '_frx',
        '_fry',
        '_frz',
        '_reference',
        '_user',
        '_tool',
    ]

    _fields_and_field_types = {
        'x': 'int32',
        'y': 'int32',
        'z': 'int32',
        'rx': 'int32',
        'ry': 'int32',
        'rz': 'int32',
        'fx': 'int32',
        'fy': 'int32',
        'fz': 'int32',
        'frx': 'int32',
        'fry': 'int32',
        'frz': 'int32',
        'reference': 'int32',
        'user': 'int32',
        'tool': 'int32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
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
        self.fx = kwargs.get('fx', int())
        self.fy = kwargs.get('fy', int())
        self.fz = kwargs.get('fz', int())
        self.frx = kwargs.get('frx', int())
        self.fry = kwargs.get('fry', int())
        self.frz = kwargs.get('frz', int())
        self.reference = kwargs.get(
            'reference', FCForceMode_Request.REFERENCE__DEFAULT)
        self.user = kwargs.get(
            'user', FCForceMode_Request.USER__DEFAULT)
        self.tool = kwargs.get(
            'tool', FCForceMode_Request.TOOL__DEFAULT)

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
        if self.fx != other.fx:
            return False
        if self.fy != other.fy:
            return False
        if self.fz != other.fz:
            return False
        if self.frx != other.frx:
            return False
        if self.fry != other.fry:
            return False
        if self.frz != other.frz:
            return False
        if self.reference != other.reference:
            return False
        if self.user != other.user:
            return False
        if self.tool != other.tool:
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

    @builtins.property
    def fx(self):
        """Message field 'fx'."""
        return self._fx

    @fx.setter
    def fx(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'fx' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'fx' field must be an integer in [-2147483648, 2147483647]"
        self._fx = value

    @builtins.property
    def fy(self):
        """Message field 'fy'."""
        return self._fy

    @fy.setter
    def fy(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'fy' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'fy' field must be an integer in [-2147483648, 2147483647]"
        self._fy = value

    @builtins.property
    def fz(self):
        """Message field 'fz'."""
        return self._fz

    @fz.setter
    def fz(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'fz' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'fz' field must be an integer in [-2147483648, 2147483647]"
        self._fz = value

    @builtins.property
    def frx(self):
        """Message field 'frx'."""
        return self._frx

    @frx.setter
    def frx(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'frx' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'frx' field must be an integer in [-2147483648, 2147483647]"
        self._frx = value

    @builtins.property
    def fry(self):
        """Message field 'fry'."""
        return self._fry

    @fry.setter
    def fry(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'fry' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'fry' field must be an integer in [-2147483648, 2147483647]"
        self._fry = value

    @builtins.property
    def frz(self):
        """Message field 'frz'."""
        return self._frz

    @frz.setter
    def frz(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'frz' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'frz' field must be an integer in [-2147483648, 2147483647]"
        self._frz = value

    @builtins.property
    def reference(self):
        """Message field 'reference'."""
        return self._reference

    @reference.setter
    def reference(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'reference' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'reference' field must be an integer in [-2147483648, 2147483647]"
        self._reference = value

    @builtins.property
    def user(self):
        """Message field 'user'."""
        return self._user

    @user.setter
    def user(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'user' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'user' field must be an integer in [-2147483648, 2147483647]"
        self._user = value

    @builtins.property
    def tool(self):
        """Message field 'tool'."""
        return self._tool

    @tool.setter
    def tool(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'tool' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'tool' field must be an integer in [-2147483648, 2147483647]"
        self._tool = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_FCForceMode_Response(type):
    """Metaclass of message 'FCForceMode_Response'."""

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
                'dobot_msgs_v4.srv.FCForceMode_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__fc_force_mode__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__fc_force_mode__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__fc_force_mode__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__fc_force_mode__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__fc_force_mode__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class FCForceMode_Response(metaclass=Metaclass_FCForceMode_Response):
    """Message class 'FCForceMode_Response'."""

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


class Metaclass_FCForceMode(type):
    """Metaclass of service 'FCForceMode'."""

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
                'dobot_msgs_v4.srv.FCForceMode')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__fc_force_mode

            from dobot_msgs_v4.srv import _fc_force_mode
            if _fc_force_mode.Metaclass_FCForceMode_Request._TYPE_SUPPORT is None:
                _fc_force_mode.Metaclass_FCForceMode_Request.__import_type_support__()
            if _fc_force_mode.Metaclass_FCForceMode_Response._TYPE_SUPPORT is None:
                _fc_force_mode.Metaclass_FCForceMode_Response.__import_type_support__()


class FCForceMode(metaclass=Metaclass_FCForceMode):
    from dobot_msgs_v4.srv._fc_force_mode import FCForceMode_Request as Request
    from dobot_msgs_v4.srv._fc_force_mode import FCForceMode_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
