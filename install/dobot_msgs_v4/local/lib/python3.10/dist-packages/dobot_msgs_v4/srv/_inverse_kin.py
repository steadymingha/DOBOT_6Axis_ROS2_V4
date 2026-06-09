# generated from rosidl_generator_py/resource/_idl.py.em
# with input from dobot_msgs_v4:srv/InverseKin.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import math  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_InverseKin_Request(type):
    """Metaclass of message 'InverseKin_Request'."""

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
                'dobot_msgs_v4.srv.InverseKin_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__inverse_kin__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__inverse_kin__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__inverse_kin__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__inverse_kin__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__inverse_kin__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class InverseKin_Request(metaclass=Metaclass_InverseKin_Request):
    """Message class 'InverseKin_Request'."""

    __slots__ = [
        '_x',
        '_y',
        '_z',
        '_rx',
        '_ry',
        '_rz',
        '_use_joint_near',
        '_joint_near',
        '_user',
        '_tool',
    ]

    _fields_and_field_types = {
        'x': 'double',
        'y': 'double',
        'z': 'double',
        'rx': 'double',
        'ry': 'double',
        'rz': 'double',
        'use_joint_near': 'string',
        'joint_near': 'string',
        'user': 'string',
        'tool': 'string',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.x = kwargs.get('x', float())
        self.y = kwargs.get('y', float())
        self.z = kwargs.get('z', float())
        self.rx = kwargs.get('rx', float())
        self.ry = kwargs.get('ry', float())
        self.rz = kwargs.get('rz', float())
        self.use_joint_near = kwargs.get('use_joint_near', str())
        self.joint_near = kwargs.get('joint_near', str())
        self.user = kwargs.get('user', str())
        self.tool = kwargs.get('tool', str())

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
        if self.use_joint_near != other.use_joint_near:
            return False
        if self.joint_near != other.joint_near:
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
                isinstance(value, float), \
                "The 'x' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'x' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._x = value

    @builtins.property
    def y(self):
        """Message field 'y'."""
        return self._y

    @y.setter
    def y(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'y' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'y' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._y = value

    @builtins.property
    def z(self):
        """Message field 'z'."""
        return self._z

    @z.setter
    def z(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'z' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'z' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._z = value

    @builtins.property
    def rx(self):
        """Message field 'rx'."""
        return self._rx

    @rx.setter
    def rx(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'rx' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'rx' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._rx = value

    @builtins.property
    def ry(self):
        """Message field 'ry'."""
        return self._ry

    @ry.setter
    def ry(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'ry' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'ry' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._ry = value

    @builtins.property
    def rz(self):
        """Message field 'rz'."""
        return self._rz

    @rz.setter
    def rz(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'rz' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'rz' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._rz = value

    @builtins.property
    def use_joint_near(self):
        """Message field 'use_joint_near'."""
        return self._use_joint_near

    @use_joint_near.setter
    def use_joint_near(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'use_joint_near' field must be of type 'str'"
        self._use_joint_near = value

    @builtins.property
    def joint_near(self):
        """Message field 'joint_near'."""
        return self._joint_near

    @joint_near.setter
    def joint_near(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'joint_near' field must be of type 'str'"
        self._joint_near = value

    @builtins.property
    def user(self):
        """Message field 'user'."""
        return self._user

    @user.setter
    def user(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'user' field must be of type 'str'"
        self._user = value

    @builtins.property
    def tool(self):
        """Message field 'tool'."""
        return self._tool

    @tool.setter
    def tool(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'tool' field must be of type 'str'"
        self._tool = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_InverseKin_Response(type):
    """Metaclass of message 'InverseKin_Response'."""

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
                'dobot_msgs_v4.srv.InverseKin_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__inverse_kin__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__inverse_kin__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__inverse_kin__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__inverse_kin__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__inverse_kin__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class InverseKin_Response(metaclass=Metaclass_InverseKin_Response):
    """Message class 'InverseKin_Response'."""

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


class Metaclass_InverseKin(type):
    """Metaclass of service 'InverseKin'."""

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
                'dobot_msgs_v4.srv.InverseKin')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__inverse_kin

            from dobot_msgs_v4.srv import _inverse_kin
            if _inverse_kin.Metaclass_InverseKin_Request._TYPE_SUPPORT is None:
                _inverse_kin.Metaclass_InverseKin_Request.__import_type_support__()
            if _inverse_kin.Metaclass_InverseKin_Response._TYPE_SUPPORT is None:
                _inverse_kin.Metaclass_InverseKin_Response.__import_type_support__()


class InverseKin(metaclass=Metaclass_InverseKin):
    from dobot_msgs_v4.srv._inverse_kin import InverseKin_Request as Request
    from dobot_msgs_v4.srv._inverse_kin import InverseKin_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
