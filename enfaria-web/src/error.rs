use crate::prelude::*;

#[derive(Debug)]
pub struct IncorrectPassword;
impl Reject for IncorrectPassword{}


#[derive(Debug)]
pub struct InvalidPassword;
impl Reject for InvalidPassword{}


#[derive(Debug)]
pub struct InvalidEmail;
impl Reject for InvalidEmail{}


#[derive(Debug)]
pub struct InvalidUsername;
impl Reject for InvalidUsername{}


#[derive(Debug)]
pub struct ExistingUser;
impl Reject for ExistingUser{}


#[derive(Debug)]
pub struct HashError;
impl Reject for HashError{}
