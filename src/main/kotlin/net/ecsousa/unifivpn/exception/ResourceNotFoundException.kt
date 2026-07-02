package net.ecsousa.unifivpn.exception

class ResourceNotFoundException(val resourceType: String, val resourceId: String):
    RuntimeException("Resource $resourceType for found for '$resourceId'")