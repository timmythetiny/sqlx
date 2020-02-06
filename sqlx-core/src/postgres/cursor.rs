use core::pin::Pin;
use core::task::{Context, Poll};

use std::future::Future;
use std::io::{self, ErrorKind::ConnectionAborted};

use futures_core::future::BoxFuture;
use futures_core::stream::BoxStream;

use crate::cursor::Cursor;
use crate::database::HasRow;
use crate::postgres::protocol::{DataRow, Message, ParameterDescription, RowDescription};
use crate::postgres::{PgConnection, PgRow, Postgres};

// 'e: the lifetime of the Executor reference
pub struct PgCursor<'e> {
    // TODO: A [PgCursor] should be constructible from a [Pool], a [PoolConnection<C>; and, a
    //       [PgConnection].
    pub(crate) connection: &'e mut PgConnection,
}

impl<'e> Cursor<'e> for PgCursor<'e> {
    type Database = Postgres;

    fn first(self) -> BoxFuture<'e, crate::Result<Option<PgRow<'e>>>> {
        todo!()
    }

    // 'c: the lifetime of the cursor
    fn next<'c: 'e>(&'c mut self) -> BoxFuture<'c, crate::Result<Option<PgRow<'c>>>> {
        todo!()
    }

    fn map<T, F>(self, f: F) -> BoxStream<'e, crate::Result<T>>
    where
        F: Fn(<Self::Database as HasRow<'e>>::Row) -> T,
    {
        todo!()
    }
}

impl<'e> Future for PgCursor<'e> {
    type Output = crate::Result<u64>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

enum Step<'s> {
    Command(u64),
    NoData,
    Row(DataRow<'s>),
    ParamDesc(Box<ParameterDescription>),
    RowDesc(Box<RowDescription>),
}

async fn step<'s>(conn: &'s mut PgConnection) -> crate::Result<Option<Step<'s>>> {
    while let Some(message) = conn.receive().await? {
        match message {
            Message::BindComplete
            | Message::ParseComplete
            | Message::PortalSuspended
            | Message::CloseComplete => {}

            Message::CommandComplete(body) => {
                return Ok(Some(Step::Command(body.affected_rows)));
            }

            Message::NoData => {
                return Ok(Some(Step::NoData));
            }

            Message::DataRow(body) => {
                return Ok(Some(Step::Row(body)));
            }

            Message::ReadyForQuery(_) => {
                conn.ready = true;

                return Ok(None);
            }

            Message::ParameterDescription(desc) => {
                return Ok(Some(Step::ParamDesc(desc)));
            }

            Message::RowDescription(desc) => {
                return Ok(Some(Step::RowDesc(desc)));
            }

            message => {
                return Err(protocol_err!("received unexpected message: {:?}", message).into());
            }
        }
    }

    // Connection was (unexpectedly) closed
    Err(io::Error::from(ConnectionAborted).into())
}
