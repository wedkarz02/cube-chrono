# Table of Contents
[[_TOC_]]

## Introduction

This document outlines the API endpoints and models offered by the backend of the application.

All api endpoints are prefixed with `/api/v1`.


## Specific Endpoints

### Hello


### Profiles

#### `GET /api/v1/profiles/logged`
- **Description**: Get information about currently logged account.
- **Headers**:
  - `Authorization` (string): JWT access token, prefixed with `Bearer `.
- **Responses**:
  - `200 OK`: Account found.
  - `401 Unauthorized`: Unauthorized to read this data.

#### `PUT /api/v1/profiles/logged/change-username`
- **Description**: Change the username of currently logged account.
- **Headers**:
  - `Authorization` (string): JWT access token, prefixed with `Bearer `.
- **Request Body**:
  - `username` (string): The new username of the account.
- **Responses**:
  - `200 OK`: Username updated.
  - `400 Bad Request`: Invalid input data.
  - `401 Unauthorized`: Unauthorized to update this data.

#### `PUT /api/v1/profiles/logged/change-password`
- **Description**: Change the password of currently logged account.
- **Headers**:
  - `Authorization` (string): JWT access token, prefixed with `Bearer `.
- **Request Body**:
  - `new_password` (string): The new password of the account.
  - `old_password` (string): Current password of the account.
- **Responses**:
  - `200 OK`: Password updated.
  - `400 Bad Request`: Invalid input data.
  - `401 Unauthorized`: Unauthorized to update this data.


### Auth

#### `POST /api/v1/auth/register`
- **Description**: Register a new account.
- **Request Body**:
  - `username` (string): The username of the new account.
  - `password` (string): The password of the new account.
- **Responses**:
  - `201 Created`: Account created.
  - `400 Bad Request`: Invalid input data.
  - `409 Conflict`: Username already taken.

#### `POST /api/v1/auth/login`
- **Description**: Authenticate the account and return JWT tokens.
- **Request Body**:
  - `username` (string): The username of the account.
  - `password` (string): The password of the account.
- **Responses**:
  - `200 OK`: Login successful, returns JWT tokens.
  - `401 Unauthorized`: Invalid credentials.

#### `POST /api/v1/auth/refresh`
- **Description**: Refresh the access token.
- **Request Body**:
  - `refresh_token` (string): The refresh token.
- **Responses**:
  - `200 OK`: Token refreshed, returns a new access token.
  - `401 Unauthorized`: Invalid or expired refresh token.

#### `POST /api/v1/auth/revoke-all-sessions`
- **Description**: Revoke all sessions of the account.
- **Request Body**:
  - `username` (string): The username of the account.
  - `password` (string): The password of the account.
- **Responses**: 
  - `200 OK`: returns number of revoked sessions.
  - `401 Unauthorized`: Invalid credentials.

#### `POST /api/v1/auth/logout`
- **Description**: Log out the current session by invalidating the refresh token.
- **Request Body**:
  - `refresh_token` (string): The refresh token.
- **Responses**:
  - `200 OK`: Logged out.
  - `401 Unauthorized`: Invalid or expired refresh token.


### Events

#### `GET /api/v1/events`
- **Description**: Get all non-private events.
- **Query Parameters**:
  - `page` (int): The page number.
  - `limit` (int): The number of events per page.
- **Responses**: 
  - `200 OK`: Returns a paginated list of non-private events.
  - `400 Bad Request`: Invalid query parameters.

#### `POST /api/v1/events`
- **Description**: Create a new event.
- **Request Body**:
  - `title` (string): The title of the event.
  - `description` (string): The description of the event.
  - `start_time` (string): The start time of the event.
  - `end_time` (string): The end time of the event.
  - `is_private` (bool): Whether the event is private.
- **Responses**: 
  - `201 Created`: Event created.
  - `400 Bad Request`: Invalid input data.
  - `401 Unauthorized`: Unauthorized to create an event.

#### `GET /api/v1/events/{event_id}`
- **Description**: Get a specific event.
- **Path Parameters**:
  - `event_id` (int): The id of the event.
- **Responses**: 
  - `200 OK`: Returns the event.
  - `404 Not Found`: Event not found.

#### `PUT /api/v1/events/{event_id}`
- **Description**: Update a specific event.
- **Path Parameters**:
  - `event_id` (int): The id of the event.
  - `title` (string): The title of the event.
  - `description` (string): The description of the event.
  - `start_time` (string): The start time of the event.
  - `end_time` (string): The end time of the event.
  - `is_private` (bool): Whether the event is private.
- **Responses**: 
  - `200 OK`: Event updated.
  - `400 Bad Request`: Invalid input data.
  - `401 Unauthorized`: Unauthorized to update the event.
  - `404 Not Found`: Event not found.

#### `DELETE /api/v1/events/{event_id}`
- **Description**: Delete a specific event.
- **Path Parameters**:
  - `event_id` (int): The id of the event.
- **Responses**: 
  - `204 No Content`: Event deleted.
  - `401 Unauthorized`: Unauthorized to delete the event.
  - `404 Not Found`: Event not found.


### Scrambles

#### `GET /api/v1/scrambles`
- **Description**: Generate a set of scrambles.
- **Query Parameters**:
  - `kind` (string): Puzzle type (possible values [Three, ...]).
  - `count` (uint8): Amount of requested scrambles.
- **Responses**:
  - `200 OK`: Event updated.
  - `400 Bad Request`: Invalid input data.
