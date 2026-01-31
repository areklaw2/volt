import type { Conversation, Message, Participant } from '@/types';

export const currentUser = {
  id: 'u-current',
  username: 'you',
  display_name: 'You',
  created_at: '2025-01-01T00:00:00Z',
};

export const users = [
  {
    id: 'u-1',
    username: 'alice',
    display_name: 'Alice Chen',
    created_at: '2025-01-01T00:00:00Z',
  },
  {
    id: 'u-2',
    username: 'bob',
    display_name: 'Bob Martinez',
    created_at: '2025-01-02T00:00:00Z',
  },
  {
    id: 'u-3',
    username: 'carol',
    display_name: 'Carol Williams',
    created_at: '2025-01-03T00:00:00Z',
  },
  {
    id: 'u-4',
    username: 'dave',
    display_name: 'Dave Kim',
    created_at: '2025-01-04T00:00:00Z',
  },
  {
    id: 'u-5',
    username: 'eve',
    display_name: 'Eve Johnson',
    created_at: '2025-01-05T00:00:00Z',
  },
];

const usersById = Object.fromEntries(
  [currentUser, ...users].map((u) => [u.id, u]),
);

export function getUserById(id: string) {
  return usersById[id];
}

// Helper to build dates relative to now
function daysAgo(d: number, hours = 0, minutes = 0): string {
  const date = new Date();
  date.setDate(date.getDate() - d);
  date.setHours(hours, minutes, 0, 0);
  return date.toISOString();
}

function toParticipant(
  user: { id: string; username: string; display_name: string },
  joined_at: string,
  last_read_at: string | null = null,
): Participant {
  return {
    user_id: user.id,
    username: user.username,
    display_name: user.display_name,
    joined_at,
    last_read_at,
  };
}

// --- Messages per conversation ---

const conv1Messages: Message[] = [
  {
    id: 'm-1-1',
    conversation_id: 'c-1',
    sender_id: 'u-1',
    content: 'Hey! Are you coming to the standup today?',
    created_at: daysAgo(1, 9, 0),
    updated_at: null,
  },
  {
    id: 'm-1-2',
    conversation_id: 'c-1',
    sender_id: 'u-current',
    content: "Yeah, I'll be there in 5 minutes",
    created_at: daysAgo(1, 9, 2),
    updated_at: null,
  },
  {
    id: 'm-1-3',
    conversation_id: 'c-1',
    sender_id: 'u-1',
    content: 'Cool. I want to discuss the new API changes',
    created_at: daysAgo(1, 9, 3),
    updated_at: null,
  },
  {
    id: 'm-1-4',
    conversation_id: 'c-1',
    sender_id: 'u-current',
    content: 'Sounds good. I pushed the PR last night',
    created_at: daysAgo(1, 9, 5),
    updated_at: null,
  },
  {
    id: 'm-1-5',
    conversation_id: 'c-1',
    sender_id: 'u-1',
    content: "Perfect, I'll review it before the meeting",
    created_at: daysAgo(1, 9, 6),
    updated_at: null,
  },
  {
    id: 'm-1-6',
    conversation_id: 'c-1',
    sender_id: 'u-1',
    content: 'Just finished reviewing. Looks great overall!',
    created_at: daysAgo(0, 10, 15),
    updated_at: null,
  },
  {
    id: 'm-1-7',
    conversation_id: 'c-1',
    sender_id: 'u-current',
    content: 'Thanks! Any comments I should address?',
    created_at: daysAgo(0, 10, 20),
    updated_at: null,
  },
  {
    id: 'm-1-8',
    conversation_id: 'c-1',
    sender_id: 'u-1',
    content: 'Left a couple of minor suggestions, nothing blocking',
    created_at: daysAgo(0, 10, 22),
    updated_at: null,
  },
];

const conv2Messages: Message[] = [
  {
    id: 'm-2-1',
    conversation_id: 'c-2',
    sender_id: 'u-2',
    content: 'Did you see the game last night?',
    created_at: daysAgo(2, 20, 0),
    updated_at: null,
  },
  {
    id: 'm-2-2',
    conversation_id: 'c-2',
    sender_id: 'u-current',
    content: 'No I missed it! What happened?',
    created_at: daysAgo(2, 20, 5),
    updated_at: null,
  },
  {
    id: 'm-2-3',
    conversation_id: 'c-2',
    sender_id: 'u-2',
    content: 'Overtime win, it was incredible',
    created_at: daysAgo(2, 20, 6),
    updated_at: null,
  },
  {
    id: 'm-2-4',
    conversation_id: 'c-2',
    sender_id: 'u-current',
    content: 'Ugh I need to watch the replay',
    created_at: daysAgo(2, 20, 10),
    updated_at: null,
  },
  {
    id: 'm-2-5',
    conversation_id: 'c-2',
    sender_id: 'u-2',
    content: "I'll send you the link",
    created_at: daysAgo(2, 20, 11),
    updated_at: null,
  },
  {
    id: 'm-2-6',
    conversation_id: 'c-2',
    sender_id: 'u-2',
    content: 'Here: check your email',
    created_at: daysAgo(2, 20, 15),
    updated_at: null,
  },
];

const conv3Messages: Message[] = [
  {
    id: 'm-3-1',
    conversation_id: 'c-3',
    sender_id: 'u-3',
    content: "Team, let's plan the sprint",
    created_at: daysAgo(3, 14, 0),
    updated_at: null,
  },
  {
    id: 'm-3-2',
    conversation_id: 'c-3',
    sender_id: 'u-1',
    content: 'I can take the auth tickets',
    created_at: daysAgo(3, 14, 5),
    updated_at: null,
  },
  {
    id: 'm-3-3',
    conversation_id: 'c-3',
    sender_id: 'u-current',
    content: "I'll handle the frontend components",
    created_at: daysAgo(3, 14, 8),
    updated_at: null,
  },
  {
    id: 'm-3-4',
    conversation_id: 'c-3',
    sender_id: 'u-4',
    content: 'I can do the database migrations',
    created_at: daysAgo(3, 14, 10),
    updated_at: null,
  },
  {
    id: 'm-3-5',
    conversation_id: 'c-3',
    sender_id: 'u-3',
    content: "Great. Let's aim to finish by Thursday",
    created_at: daysAgo(3, 14, 15),
    updated_at: null,
  },
  {
    id: 'm-3-6',
    conversation_id: 'c-3',
    sender_id: 'u-current',
    content: 'Works for me',
    created_at: daysAgo(3, 14, 16),
    updated_at: null,
  },
  {
    id: 'm-3-7',
    conversation_id: 'c-3',
    sender_id: 'u-1',
    content: 'Same here',
    created_at: daysAgo(3, 14, 17),
    updated_at: null,
  },
  {
    id: 'm-3-8',
    conversation_id: 'c-3',
    sender_id: 'u-4',
    content: 'Thursday it is!',
    created_at: daysAgo(3, 14, 18),
    updated_at: null,
  },
  {
    id: 'm-3-9',
    conversation_id: 'c-3',
    sender_id: 'u-3',
    content: 'Quick update: auth is almost done',
    created_at: daysAgo(1, 11, 0),
    updated_at: null,
  },
  {
    id: 'm-3-10',
    conversation_id: 'c-3',
    sender_id: 'u-1',
    content: "Nice! I'll push the PR today",
    created_at: daysAgo(1, 11, 5),
    updated_at: null,
  },
  {
    id: 'm-3-11',
    conversation_id: 'c-3',
    sender_id: 'u-current',
    content: 'Frontend is on track too',
    created_at: daysAgo(1, 11, 10),
    updated_at: null,
  },
  {
    id: 'm-3-12',
    conversation_id: 'c-3',
    sender_id: 'u-3',
    content: "Awesome, we're in good shape",
    created_at: daysAgo(1, 11, 12),
    updated_at: null,
  },
];

const conv4Messages: Message[] = [
  {
    id: 'm-4-1',
    conversation_id: 'c-4',
    sender_id: 'u-5',
    content: 'Welcome to the watercooler chat!',
    created_at: daysAgo(5, 10, 0),
    updated_at: null,
  },
  {
    id: 'm-4-2',
    conversation_id: 'c-4',
    sender_id: 'u-2',
    content: 'Anyone tried the new coffee machine?',
    created_at: daysAgo(4, 9, 0),
    updated_at: null,
  },
  {
    id: 'm-4-3',
    conversation_id: 'c-4',
    sender_id: 'u-current',
    content: 'Yes! The espresso is actually good now',
    created_at: daysAgo(4, 9, 15),
    updated_at: null,
  },
  {
    id: 'm-4-4',
    conversation_id: 'c-4',
    sender_id: 'u-3',
    content: 'I still prefer pour-over',
    created_at: daysAgo(4, 9, 20),
    updated_at: null,
  },
  {
    id: 'm-4-5',
    conversation_id: 'c-4',
    sender_id: 'u-5',
    content: "There's a new lunch spot on 5th street",
    created_at: daysAgo(2, 12, 0),
    updated_at: null,
  },
  {
    id: 'm-4-6',
    conversation_id: 'c-4',
    sender_id: 'u-current',
    content: 'Oh nice, what kind of food?',
    created_at: daysAgo(2, 12, 5),
    updated_at: null,
  },
  {
    id: 'm-4-7',
    conversation_id: 'c-4',
    sender_id: 'u-5',
    content: "Thai fusion, it's really good",
    created_at: daysAgo(2, 12, 10),
    updated_at: null,
  },
  {
    id: 'm-4-8',
    conversation_id: 'c-4',
    sender_id: 'u-2',
    content: "Let's go tomorrow!",
    created_at: daysAgo(2, 12, 15),
    updated_at: null,
  },
  {
    id: 'm-4-9',
    conversation_id: 'c-4',
    sender_id: 'u-4',
    content: 'Count me in',
    created_at: daysAgo(2, 12, 20),
    updated_at: null,
  },
  {
    id: 'm-4-10',
    conversation_id: 'c-4',
    sender_id: 'u-current',
    content: 'Same, sounds great',
    created_at: daysAgo(2, 12, 22),
    updated_at: null,
  },
];

export const messagesByConversation: Record<string, Message[]> = {
  'c-1': conv1Messages,
  'c-2': conv2Messages,
  'c-3': conv3Messages,
  'c-4': conv4Messages,
};

export function getLastMessage(conversationId: string): Message | undefined {
  const msgs = messagesByConversation[conversationId];
  return msgs?.[msgs.length - 1];
}

export function getUnreadCount(conversationId: string, userId: string): number {
  const conv = conversations.find((c) => c.id === conversationId);
  if (!conv) return 0;
  const participant = conv.participants.find((p) => p.user_id === userId);
  if (!participant?.last_read_at) return 0;
  const msgs = messagesByConversation[conversationId] ?? [];
  return msgs.filter(
    (m) =>
      m.sender_id !== userId &&
      new Date(m.created_at) > new Date(participant.last_read_at!),
  ).length;
}

export const conversations: Conversation[] = [
  {
    id: 'c-1',
    conversation_type: 'direct',
    name: null,
    created_at: daysAgo(7),
    updated_at: daysAgo(0, 10, 22),
    participants: [
      toParticipant(currentUser, daysAgo(7), daysAgo(0, 10, 20)),
      toParticipant(users[0], daysAgo(7)),
    ],
  },
  {
    id: 'c-2',
    conversation_type: 'direct',
    name: null,
    created_at: daysAgo(5),
    updated_at: daysAgo(2, 20, 15),
    participants: [
      toParticipant(currentUser, daysAgo(5), daysAgo(2, 20, 15)),
      toParticipant(users[1], daysAgo(5)),
    ],
  },
  {
    id: 'c-3',
    conversation_type: 'group',
    name: 'Sprint Planning',
    created_at: daysAgo(10),
    updated_at: daysAgo(1, 11, 12),
    participants: [
      toParticipant(currentUser, daysAgo(10), daysAgo(1, 11, 5)),
      toParticipant(users[0], daysAgo(10)),
      toParticipant(users[2], daysAgo(10)),
      toParticipant(users[3], daysAgo(10)),
    ],
  },
  {
    id: 'c-4',
    conversation_type: 'group',
    name: 'Watercooler',
    created_at: daysAgo(14),
    updated_at: daysAgo(2, 12, 22),
    participants: [
      toParticipant(currentUser, daysAgo(14), daysAgo(2, 12, 22)),
      toParticipant(users[1], daysAgo(14)),
      toParticipant(users[2], daysAgo(14)),
      toParticipant(users[3], daysAgo(14)),
      toParticipant(users[4], daysAgo(14)),
    ],
  },
];
