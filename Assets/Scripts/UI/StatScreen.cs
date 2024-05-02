using System;
using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.InputSystem;

public class StatScreen : MonoBehaviour
{
    private bool _active;

    [Header( "Listen to Events" )]
    [SerializeField] private VoidEventChannelSO m_OpenStatScreen;

    private void Start()
    {
        if (m_OpenStatScreen is not null)
        {
            m_OpenStatScreen.OnEventRaised += OnOpenStatScreen;
        }
    }

    public void OnOpenStatScreen()
    {
        _active = !_active;
        gameObject.transform.GetChild( 0 ).gameObject.SetActive( _active );
    }
}
