# API Style Guide

This document outlines the conventions and style for creating and maintaining API endpoints in this project. Adhering to these guidelines ensures consistency, clarity, and ease of use for both internal developers and external consumers of the API.

## 1. API Response Structure

A core principle of this API is that each endpoint is self-contained and defines its own response structure. This approach provides flexibility and avoids the constraints of a single, generic `ApiResponse<T>` wrapper.

### 1.1. Success Responses

Successful responses for each endpoint are tailored to the specific resource being returned. The structure should be intuitive and directly related to the data being requested.

#### Example: `/api/v1/auth/login`

A successful login returns a `user` object containing essential user information.

```json
{
  "user": {
    "email": "a@a.a",
    "username": "a"
  }
}
```

#### Example: `/api/v1/rooms`

A request to the rooms index endpoint returns a paginated list of rooms, including metadata about the collection.

```json
{
  "rooms": [],
  "available": 10,
  "total": 0,
  "page": 1,
  "limit": 20
}
```

### 1.2. Error Handling

Similar to success responses, error handling is defined on a per-endpoint basis. This allows for more descriptive and context-specific error messages. The HTTP status code should accurately reflect the nature of the error (e.g., `400` for bad requests, `401` for unauthorized, `422` for unprocessable entity).

#### Example: `/api/v1/auth/signup`

For validation errors during user registration, the response body includes an `errors` object. Each key in this object corresponds to a field with validation issues, and the value is an array of error messages for that field.

```json
{
  "errors": {
    "email": ["has already been taken"],
    "username": ["has already been taken"]
  }
}
```

#### Example: `/api/v1/auth/login`

In the case of invalid login credentials, a single, clear error message is returned.

```json
{
  "message": "Invalid credentials"
}
```

## 2. Naming Conventions
```rust
// TODO
```

## 3. Authentication and Authorization
```rust
// TODO
```


Note: The json examples in this document are just templates and do not reflect the actual data that will be returned from the api calls.