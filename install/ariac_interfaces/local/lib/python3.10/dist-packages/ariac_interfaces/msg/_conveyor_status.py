# generated from rosidl_generator_py/resource/_idl.py.em
# with input from ariac_interfaces:msg/ConveyorStatus.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import math  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_ConveyorStatus(type):
    """Metaclass of message 'ConveyorStatus'."""

    _CREATE_ROS_MESSAGE = None
    _CONVERT_FROM_PY = None
    _CONVERT_TO_PY = None
    _DESTROY_ROS_MESSAGE = None
    _TYPE_SUPPORT = None

    __constants = {
        'FORWARD': 0,
        'BACKWARD': 1,
    }

    @classmethod
    def __import_type_support__(cls):
        try:
            from rosidl_generator_py import import_type_support
            module = import_type_support('ariac_interfaces')
        except ImportError:
            import logging
            import traceback
            logger = logging.getLogger(
                'ariac_interfaces.msg.ConveyorStatus')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__msg__conveyor_status
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__msg__conveyor_status
            cls._CONVERT_TO_PY = module.convert_to_py_msg__msg__conveyor_status
            cls._TYPE_SUPPORT = module.type_support_msg__msg__conveyor_status
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__msg__conveyor_status

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
            'FORWARD': cls.__constants['FORWARD'],
            'BACKWARD': cls.__constants['BACKWARD'],
        }

    @property
    def FORWARD(self):
        """Message constant 'FORWARD'."""
        return Metaclass_ConveyorStatus.__constants['FORWARD']

    @property
    def BACKWARD(self):
        """Message constant 'BACKWARD'."""
        return Metaclass_ConveyorStatus.__constants['BACKWARD']


class ConveyorStatus(metaclass=Metaclass_ConveyorStatus):
    """
    Message class 'ConveyorStatus'.

    Constants:
      FORWARD
      BACKWARD
    """

    __slots__ = [
        '_direction',
        '_speed',
        '_operating_status',
    ]

    _fields_and_field_types = {
        'direction': 'uint8',
        'speed': 'double',
        'operating_status': 'uint8',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
        rosidl_parser.definition.BasicType('double'),  # noqa: E501
        rosidl_parser.definition.BasicType('uint8'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.direction = kwargs.get('direction', int())
        self.speed = kwargs.get('speed', float())
        self.operating_status = kwargs.get('operating_status', int())

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
        if self.direction != other.direction:
            return False
        if self.speed != other.speed:
            return False
        if self.operating_status != other.operating_status:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def direction(self):
        """Message field 'direction'."""
        return self._direction

    @direction.setter
    def direction(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'direction' field must be of type 'int'"
            assert value >= 0 and value < 256, \
                "The 'direction' field must be an unsigned integer in [0, 255]"
        self._direction = value

    @builtins.property
    def speed(self):
        """Message field 'speed'."""
        return self._speed

    @speed.setter
    def speed(self, value):
        if __debug__:
            assert \
                isinstance(value, float), \
                "The 'speed' field must be of type 'float'"
            assert not (value < -1.7976931348623157e+308 or value > 1.7976931348623157e+308) or math.isinf(value), \
                "The 'speed' field must be a double in [-1.7976931348623157e+308, 1.7976931348623157e+308]"
        self._speed = value

    @builtins.property
    def operating_status(self):
        """Message field 'operating_status'."""
        return self._operating_status

    @operating_status.setter
    def operating_status(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'operating_status' field must be of type 'int'"
            assert value >= 0 and value < 256, \
                "The 'operating_status' field must be an unsigned integer in [0, 255]"
        self._operating_status = value
