# generated from rosidl_generator_py/resource/_idl.py.em
# with input from dobot_msgs_v4:srv/ModbusRTUCreate.idl
# generated code does not contain a copyright notice


# Import statements for member types

import builtins  # noqa: E402, I100

import rosidl_parser.definition  # noqa: E402, I100


class Metaclass_ModbusRTUCreate_Request(type):
    """Metaclass of message 'ModbusRTUCreate_Request'."""

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
                'dobot_msgs_v4.srv.ModbusRTUCreate_Request')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__modbus_rtu_create__request
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__modbus_rtu_create__request
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__modbus_rtu_create__request
            cls._TYPE_SUPPORT = module.type_support_msg__srv__modbus_rtu_create__request
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__modbus_rtu_create__request

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class ModbusRTUCreate_Request(metaclass=Metaclass_ModbusRTUCreate_Request):
    """Message class 'ModbusRTUCreate_Request'."""

    __slots__ = [
        '_slave_id',
        '_baud',
        '_parity',
        '_data_bit',
        '_stop_bit',
    ]

    _fields_and_field_types = {
        'slave_id': 'int32',
        'baud': 'int32',
        'parity': 'string',
        'data_bit': 'int32',
        'stop_bit': 'int32',
    }

    SLOT_TYPES = (
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.UnboundedString(),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
        rosidl_parser.definition.BasicType('int32'),  # noqa: E501
    )

    def __init__(self, **kwargs):
        assert all('_' + key in self.__slots__ for key in kwargs.keys()), \
            'Invalid arguments passed to constructor: %s' % \
            ', '.join(sorted(k for k in kwargs.keys() if '_' + k not in self.__slots__))
        self.slave_id = kwargs.get('slave_id', int())
        self.baud = kwargs.get('baud', int())
        self.parity = kwargs.get('parity', str())
        self.data_bit = kwargs.get('data_bit', int())
        self.stop_bit = kwargs.get('stop_bit', int())

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
        if self.slave_id != other.slave_id:
            return False
        if self.baud != other.baud:
            return False
        if self.parity != other.parity:
            return False
        if self.data_bit != other.data_bit:
            return False
        if self.stop_bit != other.stop_bit:
            return False
        return True

    @classmethod
    def get_fields_and_field_types(cls):
        from copy import copy
        return copy(cls._fields_and_field_types)

    @builtins.property
    def slave_id(self):
        """Message field 'slave_id'."""
        return self._slave_id

    @slave_id.setter
    def slave_id(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'slave_id' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'slave_id' field must be an integer in [-2147483648, 2147483647]"
        self._slave_id = value

    @builtins.property
    def baud(self):
        """Message field 'baud'."""
        return self._baud

    @baud.setter
    def baud(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'baud' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'baud' field must be an integer in [-2147483648, 2147483647]"
        self._baud = value

    @builtins.property
    def parity(self):
        """Message field 'parity'."""
        return self._parity

    @parity.setter
    def parity(self, value):
        if __debug__:
            assert \
                isinstance(value, str), \
                "The 'parity' field must be of type 'str'"
        self._parity = value

    @builtins.property
    def data_bit(self):
        """Message field 'data_bit'."""
        return self._data_bit

    @data_bit.setter
    def data_bit(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'data_bit' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'data_bit' field must be an integer in [-2147483648, 2147483647]"
        self._data_bit = value

    @builtins.property
    def stop_bit(self):
        """Message field 'stop_bit'."""
        return self._stop_bit

    @stop_bit.setter
    def stop_bit(self, value):
        if __debug__:
            assert \
                isinstance(value, int), \
                "The 'stop_bit' field must be of type 'int'"
            assert value >= -2147483648 and value < 2147483648, \
                "The 'stop_bit' field must be an integer in [-2147483648, 2147483647]"
        self._stop_bit = value


# Import statements for member types

# already imported above
# import builtins

# already imported above
# import rosidl_parser.definition


class Metaclass_ModbusRTUCreate_Response(type):
    """Metaclass of message 'ModbusRTUCreate_Response'."""

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
                'dobot_msgs_v4.srv.ModbusRTUCreate_Response')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._CREATE_ROS_MESSAGE = module.create_ros_message_msg__srv__modbus_rtu_create__response
            cls._CONVERT_FROM_PY = module.convert_from_py_msg__srv__modbus_rtu_create__response
            cls._CONVERT_TO_PY = module.convert_to_py_msg__srv__modbus_rtu_create__response
            cls._TYPE_SUPPORT = module.type_support_msg__srv__modbus_rtu_create__response
            cls._DESTROY_ROS_MESSAGE = module.destroy_ros_message_msg__srv__modbus_rtu_create__response

    @classmethod
    def __prepare__(cls, name, bases, **kwargs):
        # list constant names here so that they appear in the help text of
        # the message class under "Data and other attributes defined here:"
        # as well as populate each message instance
        return {
        }


class ModbusRTUCreate_Response(metaclass=Metaclass_ModbusRTUCreate_Response):
    """Message class 'ModbusRTUCreate_Response'."""

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


class Metaclass_ModbusRTUCreate(type):
    """Metaclass of service 'ModbusRTUCreate'."""

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
                'dobot_msgs_v4.srv.ModbusRTUCreate')
            logger.debug(
                'Failed to import needed modules for type support:\n' +
                traceback.format_exc())
        else:
            cls._TYPE_SUPPORT = module.type_support_srv__srv__modbus_rtu_create

            from dobot_msgs_v4.srv import _modbus_rtu_create
            if _modbus_rtu_create.Metaclass_ModbusRTUCreate_Request._TYPE_SUPPORT is None:
                _modbus_rtu_create.Metaclass_ModbusRTUCreate_Request.__import_type_support__()
            if _modbus_rtu_create.Metaclass_ModbusRTUCreate_Response._TYPE_SUPPORT is None:
                _modbus_rtu_create.Metaclass_ModbusRTUCreate_Response.__import_type_support__()


class ModbusRTUCreate(metaclass=Metaclass_ModbusRTUCreate):
    from dobot_msgs_v4.srv._modbus_rtu_create import ModbusRTUCreate_Request as Request
    from dobot_msgs_v4.srv._modbus_rtu_create import ModbusRTUCreate_Response as Response

    def __init__(self):
        raise NotImplementedError('Service classes can not be instantiated')
