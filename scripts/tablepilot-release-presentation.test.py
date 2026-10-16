"""Checks for body-only release maintenance and conflict handling."""
import hashlib
import importlib.util
from pathlib import Path
import unittest
from unittest.mock import Mock

spec = importlib.util.spec_from_file_location('presentation', Path(__file__).with_name('tablepilot-release-presentation.py'))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


def item(tag='v0.3.0'):
    return {'tag': tag, 'body': 'TablePilot release.',
            'expected_body_sha256': hashlib.sha256(b'old text').hexdigest()}


class PresentationTests(unittest.TestCase):
    def test_updates_only_body(self):
        module.api = Mock(side_effect=[
            {'id': 42, 'tag_name': 'v0.3.0', 'body': 'old text'},
            {'id': 42, 'tag_name': 'v0.3.0', 'body': 'TablePilot release.'},
        ])
        module.update_descriptions([item()])
        self.assertEqual(module.api.call_args_list[1].args,
                         ('releases/42', {'body': 'TablePilot release.'}))

    def test_conflict_prevents_all_writes(self):
        module.api = Mock(side_effect=[
            {'id': 42, 'tag_name': 'v0.3.0', 'body': 'old text'},
            {'id': 43, 'tag_name': 'v0.3.1', 'body': 'edited by someone else'},
        ])
        with self.assertRaises(ValueError):
            module.update_descriptions([item(), item('v0.3.1')])
        self.assertTrue(all(len(c.args) == 1 for c in module.api.call_args_list))

    def test_repeat_skips_writes(self):
        module.api = Mock(return_value={'id': 42, 'tag_name': 'v0.3.0', 'body': 'TablePilot release.'})
        module.update_descriptions([item()])
        module.api.assert_called_once_with('releases/tags/v0.3.0')

    def test_rejects_unapproved_tag(self):
        module.api = Mock()
        with self.assertRaises(ValueError):
            module.update_descriptions([item('v0.4.1')])
        module.api.assert_not_called()


if __name__ == '__main__':
    unittest.main()
