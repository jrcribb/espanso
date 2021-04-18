/*
 * This file is part of espanso.
 *
 * Copyright (C) 2019-2021 Federico Terzi
 *
 * espanso is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * espanso is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with espanso.  If not, see <https://www.gnu.org/licenses/>.
 */

use super::super::Middleware;
use crate::engine::{
  dispatch::Mode,
  event::{
    keyboard::{Key, KeySequenceInjectRequest},
    text::{TextInjectMode, TextInjectRequest},
    Event,
  },
};

pub trait MatchInfoProvider {
  fn get_force_mode(&self, match_id: i32) -> Option<TextInjectMode>;
}

pub struct ActionMiddleware<'a> {
  match_info_provider: &'a dyn MatchInfoProvider,
}

impl<'a> ActionMiddleware<'a> {
  pub fn new(match_info_provider: &'a dyn MatchInfoProvider) -> Self {
    Self {
      match_info_provider,
    }
  }
}

impl<'a> Middleware for ActionMiddleware<'a> {
  fn name(&self) -> &'static str {
    "action"
  }

  fn next(&self, event: Event, dispatch: &mut dyn FnMut(Event)) -> Event {
    match &event {
      Event::Rendered(m_event) => Event::TextInject(TextInjectRequest {
        text: m_event.body.clone(),
        force_mode: self.match_info_provider.get_force_mode(m_event.match_id),
      }),
      Event::CursorHintCompensation(m_event) => {
        Event::KeySequenceInject(KeySequenceInjectRequest {
          keys: (0..m_event.cursor_hint_back_count)
            .map(|_| Key::ArrowLeft)
            .collect(),
        })
      }
      Event::TriggerCompensation(m_event) => {
        let mut backspace_count = m_event.trigger.chars().count();

        // We want to preserve the left separator if present
        if let Some(left_separator) = &m_event.left_separator {
          backspace_count -= left_separator.chars().count();
        }

        Event::KeySequenceInject(KeySequenceInjectRequest {
          keys: (0..backspace_count).map(|_| Key::Backspace).collect(),
        })
      }
      _ => event,
    }

    // TODO: handle images
  }
}

// TODO: test
